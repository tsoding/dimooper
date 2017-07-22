use measure::QuantMidiEvent;

/// The purpose of this struct is to be serialized or deserialized by
/// serde without implementing custom Deserialize trait, because doing
/// that comparing to rustc_serialize is more difficult.
#[derive(Serialize, Deserialize)]
pub struct SampleData {
    pub amount_of_measures: u32,
    pub buffer: Vec<QuantMidiEvent>,
    pub measure_shift: u32,
    pub quants_per_measure: u32
}
