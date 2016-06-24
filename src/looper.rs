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
    pub next_state: Option<State>,
    pub replay_buffer: Vec<MidiEvent>,
    pub overdub_buffer: Vec<MidiEvent>,
    pub next_event: usize,
    pub time_cursor: u32,
    pub out_port: &'a mut OutputPort,
}

impl<'a> Updatable for Looper<'a> {
    fn update(&mut self, delta_time: u32) {
        if self.state != State::Pause {
            if !self.replay_buffer.is_empty() {
                let t1 = self.replay_buffer[0].timestamp;
                self.time_cursor += delta_time;

                let event_timestamp = self.replay_buffer[self.next_event].timestamp - t1;
                if self.time_cursor > event_timestamp {
                    let event = self.replay_buffer[self.next_event];
                    self.out_port.write_message(event.message).unwrap();
                    self.next_event += 1;

                    if self.next_event >= self.replay_buffer.len() {
                        self.restart();
                    }
                }
            } else {
                self.restart();
            }
        }
    }
}

impl<'a> Looper<'a> {
    pub fn new(out_port: &'a mut OutputPort) -> Looper<'a> {
        Looper {
            state: State::Looping,
            next_state: None,
            replay_buffer: Vec::new(),
            overdub_buffer: Vec::new(),
            next_event: 0,
            time_cursor: 0,
            out_port: out_port,
        }
    }

    fn buffer_duration(buffer: &[MidiEvent]) -> u32 {
        let n = buffer.len();
        if n > 0 {
            buffer[n - 1].timestamp - buffer[0].timestamp
        } else {
            0
        }
    }

    fn merge_buffers(&mut self) {
        let replay_buffer_duration = Self::buffer_duration(&self.replay_buffer);
        let overdub_buffer_duration = Self::buffer_duration(&self.overdub_buffer);

        let replay_buffer_len = self.replay_buffer.len();
        let overdub_buffer_len = self.overdub_buffer.len();

        let repeat_count = (overdub_buffer_duration + replay_buffer_duration) / replay_buffer_duration;

        let replay_buffer_beginning = if !self.replay_buffer.is_empty() {
            self.replay_buffer[0].timestamp
        } else {
            0
        };

        for i in 0..repeat_count - 1 {
            for j in 0..replay_buffer_len {
                let mut event = self.replay_buffer[j].clone();
                event.timestamp += (i + 1) * replay_buffer_duration;
                self.replay_buffer.push(event);
            }
        }

        if !self.overdub_buffer.is_empty() {
            for i in 0..overdub_buffer_len {
                let mut new_event = self.overdub_buffer[i].clone();
                new_event.timestamp =
                    replay_buffer_beginning + (new_event.timestamp - self.overdub_buffer[0].timestamp);
                self.replay_buffer.push(new_event);
            }
        }

        self.replay_buffer.sort_by_key(|e| e.timestamp);
    }

    pub fn restart(&mut self) {
        match self.next_state.take() {
            Some(state) => {
                self.state = state;

                if let State::Looping = self.state {
                    if self.replay_buffer.is_empty() {
                        self.replay_buffer = self.overdub_buffer.clone();
                        self.overdub_buffer.clear();
                    } else {
                        self.merge_buffers();
                    }
                }
            },
            _ => (),
        }

        self.time_cursor = 0;
        self.next_event = 0;
    }


    pub fn reset(&mut self) {
        self.state = State::Looping;
        self.replay_buffer.clear();
        self.overdub_buffer.clear();
        self.restart();
    }

    pub fn toggle_recording(&mut self) {
        match self.state {
            State::Recording => {
                self.next_state = Some(State::Looping);
            },

            State::Looping => {
                self.state = State::Recording;
                self.overdub_buffer.clear();
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
