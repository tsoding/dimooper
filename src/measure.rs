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

    fn measure_size_millis(&self) -> u32 {
        return self.measure_size_millis
    }

    fn beat_size_millis(&self) -> u32 {
        return self.beat_size_millis
    }

    fn quant_size_millis(&self) -> u32 {
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
