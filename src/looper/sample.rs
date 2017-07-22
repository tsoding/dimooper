use sdl2::render::Renderer;

use midi;
use midi::{AbsMidiEvent, Note, MidiSink};
use measure::*;
use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[derive(Clone)]
pub struct Sample {
    // FIXME(#153): Improve performance of the event look up in sample
    pub buffer: Vec<QuantMidiEvent>,
    pub amount_of_measures: u32,
    measure_shift: u32,
    notes: Vec<Note>,
    sample_quant_length: Quant,
    quants_per_measure: Quant,
}

impl Serialize for Sample {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        unimplemented!()
        // TODO: reimplement with serde
        // s.emit_struct("Sample", 4, |s| {
        //     s.emit_struct_field("buffer", 0, |s| {
        //         self.buffer.encode(s)
        //     }).and_then(|_| {
        //         s.emit_struct_field("measure_shift", 1, |s| {
        //             s.emit_u32(self.measure_shift)
        //         })
        //     }).and_then(|_| {
        //         s.emit_struct_field("quants_per_measure", 2, |s| {
        //             self.quants_per_measure.encode(s)
        //         })
        //     }).and_then(|_| {
        //         s.emit_struct_field("amount_of_measures", 3, |s| {
        //             self.amount_of_measures.encode(s)
        //         })
        //     })
        // })
    }
}

impl<'de> Deserialize<'de> for Sample {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        unimplemented!()
        // TODO: reimplement with serde
        // d.read_struct("Sample", 3, |d| {
        //     d.read_struct_field("buffer", 0, |d| {
        //         Vec::<QuantMidiEvent>::decode(d)
        //     }).and_then(|buffer| {
        //         d.read_struct_field("measure_shift", 1, |d| {
        //             u32::decode(d)
        //         }).and_then(|measure_shift| {
        //             d.read_struct_field("quants_per_measure", 2, |d| {
        //                 Quant::decode(d)
        //             }).and_then(|quants_per_measure| {
        //                 d.read_struct_field("amount_of_measures", 3, |d| {
        //                     u32::decode(d)
        //                 }).and_then(|amount_of_measures| {
        //                     Ok(Sample::restore(buffer,
        //                                        quants_per_measure,
        //                                        measure_shift,
        //                                        amount_of_measures))
        //                 })
        //             })
        //         })
        //     })
        // })
    }
}

impl Sample {
    pub fn restore(buffer: Vec<QuantMidiEvent>,
                   quants_per_measure: Quant,
                   measure_shift: u32,
                   amount_of_measures: u32) -> Sample {
        let notes = midi::events_to_notes(&buffer);

        Sample {
            buffer: buffer,
            amount_of_measures: amount_of_measures,
            notes: notes,
            sample_quant_length: Quant(amount_of_measures) * quants_per_measure,
            quants_per_measure: quants_per_measure,
            measure_shift: measure_shift
        }
    }

    pub fn new(buffer: &[AbsMidiEvent], measure: &Measure, measure_shift: u32) -> Sample {
        let amount_of_measures = measure.amount_of_measures_in_buffer(buffer);
        let quant_buffer = measure.quantize_buffer(buffer);

        let notes = midi::events_to_notes(&quant_buffer);

        Sample {
            buffer: quant_buffer,
            amount_of_measures: amount_of_measures,
            notes: notes,
            sample_quant_length: Quant(amount_of_measures) * measure.quants_per_measure(),
            quants_per_measure: measure.quants_per_measure(),
            measure_shift: measure_shift,
        }
    }

    pub fn replay_quant<Sink: MidiSink>(&self, current_quant: Quant, sink: &mut Sink) {
        let quant_shift = Quant(self.measure_shift) * self.quants_per_measure;
        let sample_quant = (current_quant + quant_shift) % self.sample_quant_length;

        // FIXME(#153): Improve performance of the event look up in sample
        for event in &self.buffer {
            if event.quant == sample_quant {
                // FIXME(#141): Handle result of the sink message feeding
                sink.feed(event.message).unwrap();
            }
        }
    }

    fn measure_notes(&self, measure_number: u32) -> Vec<Note> {
        let start: Quant = Quant(measure_number) * self.quants_per_measure;
        let end: Quant = Quant(measure_number + 1) * self.quants_per_measure;
        let mut result = Vec::new();

        for note in &self.notes {
            if (start <= note.start_quant && note.start_quant <= end) || (start <= note.end_quant && note.end_quant <= end) {
                result.push(*note);
            }
        }

        result
    }

    pub fn render(&self, raw_measure_number: u32, renderer: &mut Renderer) {
        let current_measure_number = (raw_measure_number + self.measure_shift) % self.amount_of_measures;
        let current_measure_notes = self.measure_notes(current_measure_number);
        let note_shift = Quant(current_measure_number) * self.quants_per_measure;

        for note in &current_measure_notes {
            note.render(renderer, self.quants_per_measure, note_shift);
        }
    }
}

#[cfg(test)]
mod tests {
    use test::{Bencher};
    use super::Sample;
    use hardcode::*;
    use measure::{Quant, Measure};
    use midi::{AbsMidiEvent, TypedMidiMessage};
    use midi::DummyMidiNoteTracker;

    use serde_json;

    const DEFAULT_MEASURE: Measure = Measure {
        tempo_bpm: DEFAULT_TEMPO_BPM,
        measure_size_bpm: DEFAULT_MEASURE_SIZE_BPM,
        quantation_level: DEFAULT_QUANTATION_LEVEL,
    };

    macro_rules! test_sample_data {
        (
            $([$key: expr, $start: expr, $duration: expr]),*
        ) => {
            &[$(AbsMidiEvent {
                timestamp: $start,
                message: TypedMidiMessage::NoteOn {
                    channel: 0,
                    key: $key,
                    velocity: 0,
                }
            },

            AbsMidiEvent {
                timestamp: $start + $duration - 1,
                message: TypedMidiMessage::NoteOff {
                    channel: 0,
                    key: $key,
                    velocity: 0,
                }
            }),*]
        }
    }

    macro_rules! test_msg {
        (on => $key:expr) => {
            TypedMidiMessage::NoteOn {
                channel: 0,
                key: $key,
                velocity: 0,
            }
        };

        (off => $key:expr) => {
            TypedMidiMessage::NoteOff {
                channel: 0,
                key: $key,
                velocity: 0,
            }
        };
    }

    #[test]
    fn test_amount_of_measure_calculation() {
        let expected_amount_of_measures = 2;

        let buffer = test_sample_data! [
            [0, 0, DEFAULT_MEASURE.measure_size_millis() * expected_amount_of_measures]
        ];

        // FIXME(#156): Add Unit Tests for shifted samples
        let sample = Sample::new(buffer, &DEFAULT_MEASURE, 0);

        println!("{}", sample.amount_of_measures);

        assert_eq!(expected_amount_of_measures, sample.amount_of_measures);
    }

    #[test]
    fn test_sample_serialization() {
        let expected_amount_of_measures = 2;
        let buffer = test_sample_data! [
            [0, 0, DEFAULT_MEASURE.measure_size_millis() * expected_amount_of_measures]
        ];

        let sample = Sample::new(buffer, &DEFAULT_MEASURE, 0);

        let massaged_sample: Sample = serde_json::from_str(&serde_json::to_string(&sample).unwrap()).unwrap();

        assert_eq!(sample.buffer, massaged_sample.buffer);
        assert_eq!(sample.measure_shift, massaged_sample.measure_shift);
        assert_eq!(sample.notes, massaged_sample.notes);
        assert_eq!(sample.quants_per_measure, massaged_sample.quants_per_measure);

        assert_eq!(sample.amount_of_measures, massaged_sample.amount_of_measures);
        assert_eq!(sample.sample_quant_length, massaged_sample.sample_quant_length);
    }

    #[bench]
    fn sample_replay_quant_bench(b: &mut Bencher) {
        let quant_size_millis = DEFAULT_MEASURE.quant_size_millis();

        let buffer = test_sample_data! [
            [0, 0, quant_size_millis * 2],
            [1, 0, quant_size_millis * 2],
            [2, 0, quant_size_millis * 2],
            [0, 1 * quant_size_millis, quant_size_millis * 2],
            [1, 1 * quant_size_millis, quant_size_millis * 2],
            [2, 1 * quant_size_millis, quant_size_millis * 2],
            [0, 2 * quant_size_millis, quant_size_millis * 2],
            [1, 2 * quant_size_millis, quant_size_millis * 2],
            [2, 2 * quant_size_millis, quant_size_millis * 2],
            [0, 3 * quant_size_millis, quant_size_millis * 2],
            [1, 3 * quant_size_millis, quant_size_millis * 2],
            [2, 3 * quant_size_millis, quant_size_millis * 2],
            [0, 4 * quant_size_millis, quant_size_millis * 2],
            [1, 4 * quant_size_millis, quant_size_millis * 2],
            [2, 4 * quant_size_millis, quant_size_millis * 2],
            [0, 5 * quant_size_millis, quant_size_millis * 2],
            [1, 5 * quant_size_millis, quant_size_millis * 2],
            [2, 5 * quant_size_millis, quant_size_millis * 2],
            [0, 6 * quant_size_millis, quant_size_millis * 2],
            [1, 6 * quant_size_millis, quant_size_millis * 2],
            [2, 6 * quant_size_millis, quant_size_millis * 2]
        ];

        let sample = Sample::new(buffer, &DEFAULT_MEASURE, 0);


        b.iter(|| {
            let mut sink = DummyMidiNoteTracker;

            for _ in 0..1000 {
                for i in 0..7 {
                    sample.replay_quant(Quant(i), &mut sink)
                }
            }
        });
    }
}
