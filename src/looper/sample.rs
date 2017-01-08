use sdl2::render::Renderer;

use midi;
use midi::{AbsMidiEvent, TypedMidiMessage, Note, MidiSink};
use measure::*;
use rustc_serialize::{Decodable, Encodable, Encoder, Decoder};

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Eq)]
pub struct QuantMidiEvent {
    pub message: TypedMidiMessage,
    pub quant: Quant,
}

pub struct Sample {
    // FIXME(#153): Improve performance of the event look up in sample
    pub buffer: Vec<QuantMidiEvent>,
    pub amount_of_measures: u32,
    measure_shift: u32,
    notes: Vec<Note>,
    sample_quant_length: Quant,
    quants_per_measure: Quant,
}

impl Encodable for Sample {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Sample", 4, |s| {
            s.emit_struct_field("buffer", 0, |s| {
                self.buffer.encode(s)
            }).and_then(|_| {
                s.emit_struct_field("measure_shift", 1, |s| {
                    s.emit_u32(self.measure_shift)
                })
            }).and_then(|_| {
                s.emit_struct_field("quants_per_measure", 2, |s| {
                    self.quants_per_measure.encode(s)
                })
            }).and_then(|_| {
                s.emit_struct_field("amount_of_measures", 3, |s| {
                    self.amount_of_measures.encode(s)
                })
            })
        })
    }
}

impl Decodable for Sample {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Sample", 3, |d| {
            d.read_struct_field("buffer", 0, |d| {
                Vec::<QuantMidiEvent>::decode(d)
            }).and_then(|buffer| {
                d.read_struct_field("measure_shift", 1, |d| {
                    u32::decode(d)
                }).and_then(|measure_shift| {
                    d.read_struct_field("quants_per_measure", 2, |d| {
                        Quant::decode(d)
                    }).and_then(|quants_per_measure| {
                        d.read_struct_field("amount_of_measures", 3, |d| {
                            u32::decode(d)
                        }).and_then(|amount_of_measures| {
                            Ok(Sample::restore(buffer,
                                               quants_per_measure,
                                               measure_shift,
                                               amount_of_measures))
                        })
                    })
                })
            })
        })
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
        let quants_per_sample = Quant(amount_of_measures) * measure.quants_per_measure();

        let quant_buffer: Vec<QuantMidiEvent> = buffer.iter().map(|event| {
            QuantMidiEvent {
                message: event.message,
                quant: measure.snap_timestamp_to_quant(event.timestamp) % quants_per_sample,
            }
        }).collect();

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
    use super::Sample;
    use config::*;

    use measure::Measure;
    use midi::{AbsMidiEvent, TypedMidiMessage};

    use rustc_serialize::json;

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

        let massaged_sample: Sample = json::decode(&json::encode(&sample).unwrap()).unwrap();

        assert_eq!(sample.buffer, massaged_sample.buffer);
        assert_eq!(sample.measure_shift, massaged_sample.measure_shift);
        assert_eq!(sample.notes, massaged_sample.notes);
        assert_eq!(sample.quants_per_measure, massaged_sample.quants_per_measure);

        assert_eq!(sample.amount_of_measures, massaged_sample.amount_of_measures);
        assert_eq!(sample.sample_quant_length, massaged_sample.sample_quant_length);
    }
}
