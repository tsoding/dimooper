use midi::TypedMidiMessage;
use measure::Quant;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct QuantMidiEvent {
    pub message: TypedMidiMessage,
    pub quant: Quant,
}
