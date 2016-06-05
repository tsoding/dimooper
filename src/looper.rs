use pm::types::MidiEvent;

#[derive(PartialEq)]
pub enum State {
    Recording,
    Looping,
    Quit,
}

pub struct Looper {
    pub state: State,
    pub record_buffer: Vec<MidiEvent>,
    pub next_event: u32,
    pub dt: u32,
}

impl Default for Looper {
    fn default() -> Looper {
        Looper {
            state: State::Recording,
            record_buffer: Vec::new(),
            next_event: 0,
            dt: 0,
        }
    }
}

impl Looper {
    fn update(&mut self) {
        unimplemented!()
    }
}
