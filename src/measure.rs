#[derive(Debug)]
pub struct Measure {
    tempo_bpm: u32,
    measure_size_bpm: u32,
    quantation_level: u32,

    measure_size_millis: u32,
    beat_size_millis: u32,
    quant_size_millis: u32,
}

impl Measure {
    pub fn new(tempo_bpm: u32, measure_size_bpm: u32, quantation_level: u32) -> Measure {
        let mut measure = Measure {
            tempo_bpm: tempo_bpm,
            measure_size_bpm: measure_size_bpm,
            quantation_level: quantation_level,

            beat_size_millis: 0,
            measure_size_millis: 0,
            quant_size_millis: 0,
        };

        measure.update();

        measure
    }

    pub fn timestamp_to_quant(&self, timestamp: u32) -> u32 {
        (timestamp + self.quant_size_millis() / 2) / self.quant_size_millis()
    }

    pub fn quant_to_timestamp(&self, quant: u32) -> u32 {
        quant * self.quant_size_millis()
    }

    pub fn measure_size_millis(&self) -> u32 {
        return self.measure_size_millis
    }

    pub fn beat_size_millis(&self) -> u32 {
        return self.beat_size_millis
    }

    pub fn quant_size_millis(&self) -> u32 {
        return self.quant_size_millis
    }

    pub fn tempo_bpm(&self) -> u32 {
        return self.tempo_bpm;
    }

    pub fn measure_size_bpm(&self) -> u32 {
        return self.measure_size_bpm;
    }

    pub fn quantation_level(&self) -> u32 {
        return self.quantation_level;
    }

    pub fn update_tempo_bpm(&mut self, tempo_bpm: u32) {
        self.tempo_bpm = tempo_bpm;
        self.update();
    }

    pub fn update_measure_size_bpm(&mut self, measure_size_bpm: u32) {
        self.measure_size_bpm = measure_size_bpm;
        self.update();
    }

    pub fn update_quantation_level(&mut self, quantation_level: u32) {
        self.quantation_level = quantation_level;
        self.update();
    }

    fn update(&mut self) {
        self.beat_size_millis = (60000.0 / self.tempo_bpm as f32) as u32;
        self.measure_size_millis = self.beat_size_millis * self.measure_size_bpm;

        self.quant_size_millis = {
            let mut result = self.measure_size_millis as f32;

            for _ in 0..self.quantation_level {
                result /= self.measure_size_bpm as f32
            }

            result as u32
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Measure;

    const TEMPO_BPM: u32 = 120;
    const MEASURE_SIZE_BPM: u32 = 4;
    const QUANTATION_LEVEL: u32 = 2;

    const MEASURE_SIZE_MILLIS: u32 =  2000;
    const BEAT_SIZE_MILLIS: u32 =  500;
    const QUANT_SIZE_MILLIS: u32 =  125;


    #[test]
    fn test_measure_new() {
        let measure = Measure::new(TEMPO_BPM, MEASURE_SIZE_BPM, QUANTATION_LEVEL);

        assert_eq!(TEMPO_BPM, measure.tempo_bpm());
        assert_eq!(MEASURE_SIZE_BPM, measure.measure_size_bpm());
        assert_eq!(QUANTATION_LEVEL, measure.quantation_level());

        assert_eq!(MEASURE_SIZE_MILLIS, measure.measure_size_millis());
        assert_eq!(BEAT_SIZE_MILLIS, measure.beat_size_millis());
        assert_eq!(QUANT_SIZE_MILLIS, measure.quant_size_millis());
    }

    #[test]
    fn test_measure_update() {
        let mut measure = Measure::new(TEMPO_BPM, MEASURE_SIZE_BPM, QUANTATION_LEVEL);

        assert_eq!(MEASURE_SIZE_MILLIS, measure.measure_size_millis());
        assert_eq!(BEAT_SIZE_MILLIS, measure.beat_size_millis());
        assert_eq!(QUANT_SIZE_MILLIS, measure.quant_size_millis());

        measure.update_tempo_bpm(TEMPO_BPM + 40);

        assert_eq!(1500, measure.measure_size_millis());
        assert_eq!(375, measure.beat_size_millis());
        assert_eq!(93, measure.quant_size_millis());
    }

    #[test]
    fn test_timestamp_quant_conversion() {
        let measure = Measure::new(TEMPO_BPM, MEASURE_SIZE_BPM, QUANTATION_LEVEL);

        // timestamp to quant
        assert_eq!(0, measure.timestamp_to_quant(0));
        assert_eq!(1, measure.timestamp_to_quant(measure.quant_size_millis()));
        assert_eq!(0, measure.timestamp_to_quant(measure.quant_size_millis() / 2 - 1));
        assert_eq!(1, measure.timestamp_to_quant(measure.quant_size_millis() / 2 + 1));

        // quant to timestamp
        assert_eq!(5 * measure.quant_size_millis(), measure.quant_to_timestamp(5));
    }
}
