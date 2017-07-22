use sdl2::render::Renderer;

use midi;
use midi::{AbsMidiEvent, Note, MidiSink};
use measure::*;
use looper::SampleData;

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

impl Sample {
    pub fn as_sample_data(&self) -> SampleData {
        SampleData {
            amount_of_measures: self.amount_of_measures,
            buffer: self.buffer.clone(),
            measure_shift: self.measure_shift,
            quants_per_measure: self.quants_per_measure.as_u32()
        }
    }

    pub fn from_sample_data(sample_data: &SampleData) -> Sample {
        let notes = midi::events_to_notes(&sample_data.buffer);
        let buffer = sample_data.buffer.clone();
        let amount_of_measures = sample_data.amount_of_measures;
        let quants_per_measure = Quant(sample_data.quants_per_measure);
        let measure_shift = sample_data.measure_shift;

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
    use super::Sample;
    use hardcode::*;
    use measure::Measure;
    use midi::{AbsMidiEvent, TypedMidiMessage};

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

        let massaged_sample: Sample = Sample::from_sample_data(&serde_json::from_str(&serde_json::to_string(&sample.as_sample_data()).unwrap()).unwrap());

        assert_eq!(sample.buffer, massaged_sample.buffer);
        assert_eq!(sample.measure_shift, massaged_sample.measure_shift);
        assert_eq!(sample.notes, massaged_sample.notes);
        assert_eq!(sample.quants_per_measure, massaged_sample.quants_per_measure);

        assert_eq!(sample.amount_of_measures, massaged_sample.amount_of_measures);
        assert_eq!(sample.sample_quant_length, massaged_sample.sample_quant_length);
    }
}
