use pm::types::MidiEvent;
use pm::OutputPort;
use ::updatable::Updatable;

#[derive(PartialEq)]
pub enum State {
    Recording,
    Looping,
    Quit,
}

pub struct Looper<'a> {
    pub state: State,
    pub record_buffer: Vec<MidiEvent>,
    pub next_event: usize,
    pub time_cursor: u32,
    pub out_port: &'a mut OutputPort,
}

impl<'a> Updatable for Looper<'a> {
    fn update(&mut self, delta_time: u32) {
        if let State::Looping = self.state {
            if !self.record_buffer.is_empty() {
                let t1 = self.record_buffer[0].timestamp;
                self.time_cursor += delta_time;

                let event_timestamp = self.record_buffer[self.next_event].timestamp - t1;
                if self.time_cursor > event_timestamp {
                    let event = self.record_buffer[self.next_event];
                    self.out_port.write_message(event.message).unwrap();
                    self.next_event += 1;

                    if self.next_event >= self.record_buffer.len() {
                        self.reset();
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
            next_event: 0,
            time_cursor: 0,
            out_port: out_port,
        }
    }

    pub fn reset(&mut self) {
        self.time_cursor = 0;
        self.next_event = 0;
    }

    pub fn looping(&mut self) {
        self.state = State::Looping;
        if !self.record_buffer.is_empty() {
            self.reset();
        }
    }

    pub fn on_midi_event(&mut self, event: &MidiEvent) {
        if let State::Recording = self.state {
            if ::midi::is_note_message(&event.message) {
                self.record_buffer.push(event.clone());
            }
        }
    }
}
