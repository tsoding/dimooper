use sdl2::render::Renderer;

use midi;
use midi::{AbsMidiEvent, TypedMidiMessage, Note};
use measure::{Measure, Quant};
use traits::Renderable;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuantMidiEvent {
    pub message: TypedMidiMessage,
    pub quant: Quant,
}

pub struct Sample {
    pub buffer: Vec<QuantMidiEvent>,
    pub amount_of_measures: u32,
    notes: Vec<Note>,
    time_cursor: u32,
    measure: Measure,
}

impl Sample {
    fn amount_of_measures_in_buffer(buffer: &[AbsMidiEvent], measure: &Measure) -> u32 {
        let n = buffer.len();

        if n > 0 {
            (buffer[n - 1].timestamp - buffer[0].timestamp + measure.measure_size_millis()) / measure.measure_size_millis()
        } else {
            1
        }
    }

    pub fn new(buffer: &[AbsMidiEvent], measure: &Measure) -> Sample {
        let amount_of_measures = Self::amount_of_measures_in_buffer(&buffer, &measure);

        let quant_buffer = {
            let mut result = Vec::new();

            for event in buffer {
                result.push(QuantMidiEvent {
                    message: event.message,
                    quant: measure.timestamp_to_quant(event.timestamp),
                })
            }

            result
        };

        let notes = midi::events_to_notes(&quant_buffer);

        Sample {
            buffer: quant_buffer,
            amount_of_measures: amount_of_measures,
            time_cursor: amount_of_measures * measure.measure_size_millis(),
            measure: measure.clone(),
            notes: notes,
        }
    }

    pub fn update_measure(&mut self, new_measure: &Measure) {
        self.time_cursor =
            self.measure.scale_time_cursor(new_measure,
                                           self.amount_of_measures,
                                           self.time_cursor);

        self.measure = new_measure.clone();
    }

    pub fn get_next_messages(&mut self, delta_time: u32) -> Vec<TypedMidiMessage> {
        let next_time_cursor = self.time_cursor + delta_time;
        let sample_size_millis = self.measure.measure_size_millis() * self.amount_of_measures;
        let mut result = Vec::new();

        self.gather_events_in_timerange(&mut result, self.time_cursor + 1, next_time_cursor);
        self.time_cursor = next_time_cursor % sample_size_millis;

        if next_time_cursor >= sample_size_millis {
            self.gather_events_in_timerange(&mut result, 0, self.time_cursor);
        }

        result.iter().map(|e| e.message).collect()
    }

    fn gather_events_in_timerange(&self, result: &mut Vec<QuantMidiEvent>, start: u32, end: u32) {
        for event in self.buffer.iter() {
            let timestamp = self.measure.quant_to_timestamp(event.quant);
            if start <= timestamp && timestamp <= end {
                result.push(event.clone());
            }
        }
    }

    fn measure_notes(&self, measure_number: u32) -> Vec<Note> {
        let measure_size_millis = self.measure.measure_size_millis();
        let start: u32 = measure_number * measure_size_millis;
        let end: u32 = (measure_number + 1) * measure_size_millis;
        let mut result = Vec::new();

        for note in &self.notes {
            let note_start_abs = self.measure.quant_to_timestamp(note.start_quant);
            let note_end_abs = self.measure.quant_to_timestamp(note.end_quant);
            if start <= note_start_abs && note_start_abs <= end && start <= note_end_abs && note_end_abs <= end {
                result.push(note.clone());
            }
        }

        result
    }

    pub fn quants_per_sample(&self) -> Quant {
        self.measure.quants_per_measure() * Quant(self.amount_of_measures)
    }
}

impl Renderable for Sample {
    fn render(&self, renderer: &mut Renderer) {
        let measure_size_millis = self.measure.measure_size_millis();
        let current_measure_number = self.time_cursor / measure_size_millis;
        let current_measure_notes = self.measure_notes(current_measure_number);
        let note_shift = Quant(current_measure_number) * self.measure.quants_per_measure();

        for note in &current_measure_notes {
            Note {
                start_quant: note.start_quant - note_shift,
                end_quant: note.end_quant - note_shift,
                .. *note
            }.render(renderer, self.measure.quants_per_measure());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sample;
    use config::*;

    use measure::{Measure, Quant};
    use midi::{AbsMidiEvent, TypedMidiMessage};

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
    fn test_quants_per_sample() {
        let expected_amount_of_measures = 2;

        let buffer = test_sample_data! [
            [0, 0, DEFAULT_MEASURE.measure_size_millis() * expected_amount_of_measures]
        ];

        let sample = Sample::new(buffer, &DEFAULT_MEASURE);

        assert_eq!(DEFAULT_MEASURE.quants_per_measure() * Quant(expected_amount_of_measures),
                   sample.quants_per_sample());
    }

    #[test]
    fn test_get_next_messages() {
        let buffer = test_sample_data! [
            [1,
             0,
             DEFAULT_MEASURE.measure_size_millis()],
            [2,
             DEFAULT_MEASURE.measure_size_millis() + DEFAULT_MEASURE.quant_size_millis(),
             DEFAULT_MEASURE.measure_size_millis() - DEFAULT_MEASURE.quant_size_millis()]
        ];

        let test_data = &[
            (DEFAULT_MEASURE.measure_size_millis(),
             vec![
                test_msg!(on => 1),
                test_msg!(off => 1),
             ]),

            (DEFAULT_MEASURE.measure_size_millis() / 2,
             vec![test_msg!(on => 2)]),

            (DEFAULT_MEASURE.measure_size_millis(),
             vec![
                 test_msg!(off => 2),
                 test_msg!(on => 1),
             ]),
        ];

        let mut sample = Sample::new(buffer, &DEFAULT_MEASURE);
        assert_eq!(2, sample.amount_of_measures);

        for &(delta_time, ref expected_messages) in test_data {
            let messages = sample.get_next_messages(delta_time);
            assert_eq!(expected_messages, &messages);
        }

    }

    #[test]
    fn test_amount_of_measure_calculation() {
        let expected_amount_of_measures = 2;

        let buffer = test_sample_data! [
            [0, 0, DEFAULT_MEASURE.measure_size_millis() * expected_amount_of_measures]
        ];

        let sample = Sample::new(buffer, &DEFAULT_MEASURE);

        println!("{}", sample.amount_of_measures);

        assert_eq!(expected_amount_of_measures, sample.amount_of_measures);
    }
}
