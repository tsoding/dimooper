use midi::TypedMidiMessage;
use measure::Quant;

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Eq)]
pub struct QuantMidiEvent {
    pub message: TypedMidiMessage,
    pub quant: Quant,
}
