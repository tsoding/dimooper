#[derive(Debug)]
pub struct Measure {
    pub tempo_bpm: u32,
    pub measure_size_bpm: u32,
    pub quantation_level: u32,

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

    pub fn measure_size_millis(&self) -> u32 {
        return self.measure_size_millis
    }

    pub fn beat_size_millis(&self) -> u32 {
        return self.beat_size_millis
    }

    pub fn quant_size_millis(&self) -> u32 {
        return self.quant_size_millis
    }

    pub fn update(&mut self) {
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

        assert_eq!(TEMPO_BPM, measure.tempo_bpm);
        assert_eq!(MEASURE_SIZE_BPM, measure.measure_size_bpm);
        assert_eq!(QUANTATION_LEVEL, measure.quantation_level);

        assert_eq!(MEASURE_SIZE_MILLIS, measure.measure_size_millis());
        assert_eq!(BEAT_SIZE_MILLIS, measure.beat_size_millis());
        assert_eq!(QUANT_SIZE_MILLIS, measure.quant_size_millis());
    }

    #[test]
    fn test_measure_update() {
        let mut measure = Measure::new(TEMPO_BPM, MEASURE_SIZE_BPM, QUANTATION_LEVEL);

        measure.tempo_bpm = TEMPO_BPM + 40;

        assert_eq!(MEASURE_SIZE_MILLIS, measure.measure_size_millis());
        assert_eq!(BEAT_SIZE_MILLIS, measure.beat_size_millis());
        assert_eq!(QUANT_SIZE_MILLIS, measure.quant_size_millis());

        measure.update();

        assert_eq!(1500, measure.measure_size_millis());
        assert_eq!(375, measure.beat_size_millis());
        assert_eq!(93, measure.quant_size_millis());
    }
}
