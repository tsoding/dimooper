use midi::{TypedMidiEvent, TypedMidiMessage};
use measure::{Measure, Quant};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuantMidiEvent {
    pub message: TypedMidiMessage,
    pub quant: Quant,
}

pub struct Sample {
    pub buffer: Vec<QuantMidiEvent>,
    pub amount_of_measures: u32,
    time_cursor: u32,
}

impl Sample {
    fn amount_of_measures_in_buffer(buffer: &[TypedMidiEvent], measure: &Measure) -> u32 {
        let n = buffer.len();

        if n > 0 {
            (buffer[n - 1].timestamp - buffer[0].timestamp + measure.measure_size_millis()) / measure.measure_size_millis()
        } else {
            1
        }
    }

    pub fn new(buffer: &[TypedMidiEvent], measure: &Measure) -> Sample {
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

        Sample {
            buffer: quant_buffer,
            amount_of_measures: amount_of_measures,
            time_cursor: 0,
        }
    }

    pub fn get_next_messages(&mut self, delta_time: u32, measure: &Measure) -> Vec<TypedMidiMessage> {
        let next_time_cursor = self.time_cursor + delta_time;
        let sample_size_millis = measure.measure_size_millis() * self.amount_of_measures;
        let mut result = Vec::new();

        self.gather_messages_in_timerange(measure, &mut result, self.time_cursor, next_time_cursor);
        self.time_cursor = next_time_cursor % sample_size_millis;

        if next_time_cursor >= sample_size_millis {
            self.gather_messages_in_timerange(measure, &mut result, 0, self.time_cursor);
        }

        result
    }

    fn gather_messages_in_timerange(&self, measure: &Measure, result: &mut Vec<TypedMidiMessage>, start: u32, end: u32) {
        for event in self.buffer.iter() {
            let timestamp = measure.quant_to_timestamp(event.quant);
            if start <= timestamp && timestamp <= end {
                result.push(event.message);
            }
        }
    }
}
