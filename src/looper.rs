use pm::types::MidiEvent;
use pm::OutputPort;
use updatable::Updatable;

#[derive(PartialEq)]
pub enum State {
    Recording,
    Looping,
    Pause,
}

pub struct Looper<'a> {
    pub state: State,
    pub record_buffer: Vec<MidiEvent>,
    pub overdub_buffer: Vec<MidiEvent>,
    pub next_event: usize,
    pub record_start: u32,
    pub time_cursor: u32,
    pub out_port: &'a mut OutputPort,
}

impl<'a> Updatable for Looper<'a> {
    fn update(&mut self, delta_time: u32) {
        if self.state != State::Pause {
            if !self.record_buffer.is_empty() {
                let t1 = self.record_buffer[0].timestamp;
                self.time_cursor += delta_time;

                let event_timestamp = self.record_buffer[self.next_event].timestamp - t1;
                if self.time_cursor > event_timestamp {
                    let event = self.record_buffer[self.next_event];
                    self.out_port.write_message(event.message).unwrap();
                    self.next_event += 1;

                    if self.next_event >= self.record_buffer.len() {
                        self.restart();
                    }
                }
            }
        }
    }
}

impl<'a> Looper<'a> {
    pub fn new(out_port: &'a mut OutputPort) -> Looper<'a> {
        Looper {
            state: State::Recording,
            record_buffer: Vec::new(),
            overdub_buffer: Vec::new(),
            next_event: 0,
            record_start: 0,
            time_cursor: 0,
            out_port: out_port,
        }
    }

    pub fn restart(&mut self) {
        self.time_cursor = 0;
        self.next_event = 0;
    }


    pub fn reset(&mut self) {
        self.state = State::Recording;
        self.record_buffer.clear();
        self.restart();
    }

    pub fn toggle_recording(&mut self) {
        match self.state {
            State::Recording => {
                self.state = State::Looping;
                if !self.record_buffer.is_empty() {
                    self.restart();
                }
            },

            State::Looping => {
                self.state = State::Recording;
                self.overdub_buffer.clear();
                self.record_start = self.time_cursor;
            }

            _ => ()
        }

    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            State::Looping => self.state = State::Pause,
            State::Pause => self.state = State::Looping,
            _ => (),
        }
    }

    pub fn on_midi_event(&mut self, event: &MidiEvent) {
        if ::midi::is_note_message(&event.message) {
            match self.state {
                State::Recording => self.overdub_buffer.push(event.clone()),
                _ => (),
            }
        }

        self.out_port.write_message(event.message).unwrap();
    }
}
