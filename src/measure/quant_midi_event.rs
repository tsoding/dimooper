use midi::TypedMidiMessage;
use measure::Quant;

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Eq, Clone)]
pub struct QuantMidiEvent {
    pub message: TypedMidiMessage,
    pub quant: Quant,
}
