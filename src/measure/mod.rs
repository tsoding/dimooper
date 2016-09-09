mod quant;

pub use self::quant::Quant;

// FIXME(#142): measure should have only converters
// make all of the fields private
#[derive(Debug, Clone)]
pub struct Measure {
    pub tempo_bpm: u32,
    pub measure_size_bpm: u32,
    pub quantation_level: u32,
}

impl Measure {
    /// Snaps the timestamp to the closest quant
    pub fn snap_timestamp_to_quant(&self, timestamp: u32) -> Quant {
        Quant((timestamp + self.quant_size_millis() / 2) / self.quant_size_millis())
    }

    pub fn timestamp_to_quant(&self, timestamp: u32) -> Quant {
        Quant(timestamp / self.quant_size_millis())
    }

    pub fn timestamp_to_measure(&self, timestamp: u32) -> u32 {
        timestamp / self.measure_size_millis()
    }

    pub fn measures_to_quants(&self, measures: u32) -> Quant {
        Quant(measures) * self.quants_per_measure()
    }

    pub fn quant_to_timestamp(&self, Quant(quant_value): Quant) -> u32 {
        quant_value * self.quant_size_millis()
    }

    // FIXME(#142): measure should have only converters
    // Get rid of this or make private
    pub fn measure_size_millis(&self) -> u32 {
        self.beat_size_millis() * self.measure_size_bpm
    }

    // FIXME(#142): measure should have only converters
    // Get rid of this or make private
    pub fn beat_size_millis(&self) -> u32 {
        (60000.0 / self.tempo_bpm as f32) as u32
    }

    // FIXME(#142): measure should have only converters
    // Get rid of this or make private
    pub fn quants_per_measure(&self) -> Quant {
        Quant(self.measure_size_bpm.pow(self.quantation_level))
    }

    pub fn quant_size_millis(&self) -> u32 {
        let mut result = self.measure_size_millis() as f32;

        for _ in 0..self.quantation_level {
            result /= self.measure_size_bpm as f32
        }

        result as u32
    }

    pub fn scale_time_cursor(&self, new_measure: &Measure, amount_of_measures: u32, time_cursor: u32) -> u32 {
        let s0 = (amount_of_measures * self.measure_size_millis()) as f32;
        let s1 = (amount_of_measures * new_measure.measure_size_millis()) as f32;
        (time_cursor as f32 / s0 * s1) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::{Measure, Quant};

    const TEMPO_BPM: u32 = 120;
    const MEASURE_SIZE_BPM: u32 = 4;
    const QUANTATION_LEVEL: u32 = 2;

    const MEASURE_SIZE_MILLIS: u32 =  2000;
    const BEAT_SIZE_MILLIS: u32 =  500;
    const QUANT_SIZE_MILLIS: u32 =  125;

    const MEASURE: Measure = Measure {
        tempo_bpm: TEMPO_BPM,
        measure_size_bpm: MEASURE_SIZE_BPM,
        quantation_level: QUANTATION_LEVEL,
    };

    #[test]
    fn test_measure_new() {
        assert_eq!(TEMPO_BPM, MEASURE.tempo_bpm);
        assert_eq!(MEASURE_SIZE_BPM, MEASURE.measure_size_bpm);
        assert_eq!(QUANTATION_LEVEL, MEASURE.quantation_level);

        assert_eq!(MEASURE_SIZE_MILLIS, MEASURE.measure_size_millis());
        assert_eq!(BEAT_SIZE_MILLIS, MEASURE.beat_size_millis());
        assert_eq!(QUANT_SIZE_MILLIS, MEASURE.quant_size_millis());
    }

    #[test]
    fn test_measure_update() {
        assert_eq!(MEASURE_SIZE_MILLIS, MEASURE.measure_size_millis());
        assert_eq!(BEAT_SIZE_MILLIS, MEASURE.beat_size_millis());
        assert_eq!(QUANT_SIZE_MILLIS, MEASURE.quant_size_millis());

        let updated_measure = Measure { tempo_bpm: TEMPO_BPM + 40, .. MEASURE };

        assert_eq!(1500, updated_measure.measure_size_millis());
        assert_eq!(375, updated_measure.beat_size_millis());
        assert_eq!(93, updated_measure.quant_size_millis());
    }

    #[test]
    fn test_snap_timestamp_to_quant() {
        assert_eq!(Quant(0), MEASURE.snap_timestamp_to_quant(0));
        assert_eq!(Quant(1), MEASURE.snap_timestamp_to_quant(MEASURE.quant_size_millis()));
        assert_eq!(Quant(0), MEASURE.snap_timestamp_to_quant(MEASURE.quant_size_millis() / 2 - 1));
        assert_eq!(Quant(1), MEASURE.snap_timestamp_to_quant(MEASURE.quant_size_millis() / 2 + 1));
    }

    #[test]
    fn test_quant_to_timestamp() {
        assert_eq!(5 * MEASURE.quant_size_millis(), MEASURE.quant_to_timestamp(Quant(5)));
    }

    #[test]
    fn test_timestamp_to_quant() {
        assert_eq!(Quant(0), MEASURE.timestamp_to_quant(0));
        assert_eq!(Quant(1), MEASURE.timestamp_to_quant(MEASURE.quant_size_millis()));
        assert_eq!(Quant(0), MEASURE.timestamp_to_quant(MEASURE.quant_size_millis() / 2 - 1));
        assert_eq!(Quant(0), MEASURE.timestamp_to_quant(MEASURE.quant_size_millis() / 2 + 1));
    }

    #[test]
    fn test_timestamp_to_measure() {
        assert_eq!(0, MEASURE.timestamp_to_measure(0));
        assert_eq!(1, MEASURE.timestamp_to_measure(MEASURE.measure_size_millis()));
        assert_eq!(0, MEASURE.timestamp_to_measure(MEASURE.measure_size_millis() - 1));
    }

    #[test]
    fn test_quants_per_measure() {
        assert_eq!(Quant(16), MEASURE.quants_per_measure());
    }

    #[test]
    fn test_scale_time_cursor() {
        let amount_of_measures = 2;
        let time_cursor = MEASURE.measure_size_millis();

        let new_measure = Measure { tempo_bpm: TEMPO_BPM + 45, .. MEASURE };

        assert_eq!(new_measure.measure_size_millis(),
                   MEASURE.scale_time_cursor(&new_measure,
                                             amount_of_measures,
                                             time_cursor))
    }
}
