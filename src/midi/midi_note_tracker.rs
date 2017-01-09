use midi::MidiSink;

pub trait MidiNoteTracker : MidiSink {
    fn close_opened_notes(&mut self);
}
