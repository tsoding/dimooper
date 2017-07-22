use looper::Sample;
use measure::Measure;

use serde::{Deserialize, Serialize, Serializer, Deserializer};

pub struct Composition {
    pub samples: Vec<Sample>,
    pub measure: Measure,
}

impl Composition {
    pub fn new(measure: Measure, samples: Vec<Sample>) -> Composition {
        Composition {
            measure: measure,
            samples: samples
        }
    }
}

impl Serialize for Composition {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        unimplemented!()
        // TODO: reimplement with serde
        // s.emit_struct("Composition", 2, |s| {
        //     s.emit_struct_field("measure", 0, |s| {
        //         self.measure.encode(s)
        //     }).and_then(|_| {
        //         s.emit_struct_field("samples", 1, |s| {
        //             self.samples.encode(s)
        //         })
        //     })
        // })
    }
}

impl<'de> Deserialize<'de> for Composition {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        unimplemented!()
        // TODO: reimplement with serde
        // d.read_struct("Looper", 2, |d| {
        //     let measure_field = d.read_struct_field("measure", 0, |d| {
        //         Measure::decode(d)
        //     });

        //     let samples_field = d.read_struct_field("samples", 1, |d| {
        //         Vec::<Sample>::decode(d)
        //     });

        //     measure_field.and_then(|measure| {
        //         samples_field.and_then(|samples| {
        //             Ok(Self::new(measure, samples))
        //         })
        //     })
        // })
    }
}

#[cfg(test)]
mod tests {
    use super::Composition;
    use hardcode::*;
    use measure::Measure;
    use looper::Sample;
    use serde::json;
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

    #[test]
    fn test_composition_serialization() {
        let expected_amount_of_measures = 2;
        let buffer = test_sample_data! [
            [0, 0, DEFAULT_MEASURE.measure_size_millis() * expected_amount_of_measures]
        ];

        let samples = vec![
            Sample::new(buffer, &DEFAULT_MEASURE, 0),
            Sample::new(buffer, &DEFAULT_MEASURE, 0)
        ];

        let composition = Composition::new(DEFAULT_MEASURE, samples);
        let massaged_composition: Composition = json::decode(&json::encode(&composition).unwrap()).unwrap();

        assert_eq!(composition.measure, massaged_composition.measure)
    }
}
