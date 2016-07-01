use pm::OutputPort;
use midi::TypedMidiEvent;
use midi;

use updatable::Updatable;
use renderable::Renderable;
use graphicsprimitives::CircleRenderer;

use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

#[derive(PartialEq)]
pub enum State {
    Recording,
    Looping,
    Pause,
}

pub struct Looper<'a> {
    pub state: State,
    pub next_state: Option<State>,
    pub replay_buffer: Vec<TypedMidiEvent>,
    pub overdub_buffer: Vec<TypedMidiEvent>,
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

macro_rules! colors {
    ($($hex:expr),*) => {
        &[$(
            Color::RGB((($hex & 0xFF0000) >> 16) as u8,
                       (($hex & 0xFF00) >> 8) as u8,
                       ($hex & 0xFF) as u8)
        ),*]
    }
}

const CHANNEL_PALETTE: &'static [Color; 5] = colors![0xF15A5A, 0xF0C419, 0x4EBA6F, 0x2D95BF,
                                                     0x955BA5];

fn multiply_color_vector(color: Color, factor: f32) -> Color {
    match color {
        Color::RGB(r, g, b) | Color::RGBA(r, g, b, _) => {
            Color::RGB((r as f32 * factor) as u8,
                       (g as f32 * factor) as u8,
                       (b as f32 * factor) as u8)
        }
    }
}

impl<'a> Renderable for Looper<'a> {
    fn render(&self, renderer: &mut Renderer) {
        let window_width = renderer.viewport().width();
        let window_height = renderer.viewport().height();

        if self.replay_buffer.len() > 1 {
            let row_height = window_height as f32 / 128.0;
            let n = self.replay_buffer.len();
            let dt = (self.replay_buffer[n - 1].timestamp - self.replay_buffer[0].timestamp) as f32;

            let replay_buffer = &self.replay_buffer;
            let notes = midi::events_to_notes(replay_buffer);

            for note in notes {
                let brightness_factor =  note.velocity as f32 / 127.0;
                let base_color = CHANNEL_PALETTE[note.channel as usize % CHANNEL_PALETTE.len()];
                let color = multiply_color_vector(base_color, brightness_factor);

                let t1 = (note.start_timestamp - replay_buffer[0].timestamp) as f32;
                let t2 = (note.end_timestamp - replay_buffer[0].timestamp) as f32;
                let x1 = (t1 / dt * (window_width as f32 - 10.0) + 5.0) as i32;
                let x2 = (t2 / dt * (window_width as f32 - 10.0) + 5.0) as i32;
                let y = (row_height * (127 - note.key) as f32) as i32;

                renderer.set_draw_color(color);
                renderer.fill_rect(Rect::new(x1, y, (x2 - x1 + 1) as u32, row_height as u32)).unwrap();
            }

            let x = ((self.time_cursor as f32) / dt * (window_width as f32 - 10.0) + 5.0) as i32;
            renderer.set_draw_color(Color::RGB(255, 255, 255));
            renderer.draw_line(Point::from((x, 0)),
                               Point::from((x, window_height as i32))).unwrap();

        }

        let r = 15;
        let p = 25;
        let x = window_width as i32 - r - 2 * p;
        let y = r + p;
        renderer.set_draw_color(Color::RGB(255, 0, 0));

        if let State::Recording = self.state {
            renderer.fill_circle(x, y, r);
        } else {
            renderer.draw_circle(x, y, r);
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

    fn buffer_duration(buffer: &[TypedMidiEvent]) -> u32 {
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

        let repeat_count = (overdub_buffer_duration + replay_buffer_duration) /
                           replay_buffer_duration;

        let replay_buffer_beginning = if !self.replay_buffer.is_empty() {
            self.replay_buffer[0].timestamp
        } else {
            0
        };

        for i in 0..repeat_count - 1 {
            for j in 0..replay_buffer_len {
                let mut event = self.replay_buffer[j];
                event.timestamp += (i + 1) * replay_buffer_duration;
                self.replay_buffer.push(event);
            }
        }

        for mut event in self.overdub_buffer.iter().cloned() {
            event.timestamp = replay_buffer_beginning +
                              (event.timestamp - self.overdub_buffer[0].timestamp);
            self.replay_buffer.push(event);
        }

        self.replay_buffer.sort_by_key(|e| e.timestamp);
    }

    pub fn restart(&mut self) {
        if let Some(state) = self.next_state.take() {
            self.state = state;

            if let State::Looping = self.state {
                if self.replay_buffer.is_empty() {
                    self.replay_buffer = self.overdub_buffer.clone();
                    self.overdub_buffer.clear();
                } else {
                    self.merge_buffers();
                }
            }
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
            }

            State::Looping => {
                self.state = State::Recording;
                self.overdub_buffer.clear();
            }

            _ => (),
        }

    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            State::Looping => self.state = State::Pause,
            State::Pause => self.state = State::Looping,
            _ => (),
        }
    }

    pub fn on_midi_event(&mut self, event: &TypedMidiEvent) {
        if let State::Recording = self.state {
            self.overdub_buffer.push(*event);
        }

        self.out_port.write_message(event.message).unwrap();
    }
}
