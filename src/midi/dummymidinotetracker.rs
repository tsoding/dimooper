use pm::types::Result;
use midi::{TypedMidiMessage, MidiSink, MidiNoteTracker};

pub struct DummyMidiNoteTracker;

impl MidiNoteTracker for DummyMidiNoteTracker {
    fn close_opened_notes(&mut self) {}
}

impl MidiSink for DummyMidiNoteTracker {
    fn feed(&mut self, midi_message: TypedMidiMessage) -> Result<()> {
        Ok(())
    }
}
