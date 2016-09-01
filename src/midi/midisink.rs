use pm::types::Result;
use midi::TypedMidiMessage;

pub trait MidiSink {
    fn feed(&mut self, midi_message: TypedMidiMessage) -> Result<()>;
}
