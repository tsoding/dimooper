use looper::{SampleData};
use measure::Measure;

// TODO: rename Composition to CompositionData.
//
// Because it has the same purpose as SampleData
#[derive(Serialize, Deserialize)]
pub struct Composition {
    pub samples: Vec<SampleData>,
    pub measure: Measure,
}

#[cfg(test)]
mod tests {
    use super::Composition;
    use hardcode::*;
    use measure::Measure;
    use looper::Sample;
    use serde_json;
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

        let composition = Composition {
            measure: DEFAULT_MEASURE,
            samples: samples.iter().map(|sample| sample.as_sample_data()).collect(),
        };
        let massaged_composition: Composition = serde_json::from_str(&serde_json::to_string(&composition).unwrap()).unwrap();

        assert_eq!(composition.measure, massaged_composition.measure)
    }
}
