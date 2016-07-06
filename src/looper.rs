use pm::OutputPort;
use midi::{TypedMidiEvent, TypedMidiMessage};
use config::*;

use updatable::Updatable;
use renderable::Renderable;
use graphicsprimitives::CircleRenderer;

use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;

#[derive(PartialEq)]
pub enum State {
    Recording,
    Looping,
    Pause,
}

pub struct Sample {
    pub buffer: Vec<TypedMidiEvent>,
    amount_of_measures: u32,
    measure_size_millis: u32,
    time_cursor: u32,
}

impl Sample {

    fn amount_of_measures_in_buffer(buffer: &[TypedMidiEvent], measure_size_millis: u32) -> u32 {
        let n = buffer.len();

        if n > 0 {
            (buffer[n - 1].timestamp - buffer[0].timestamp + measure_size_millis) / measure_size_millis
        } else {
            1
        }
    }

    pub fn new(buffer: Vec<TypedMidiEvent>, measure_size_millis: u32) -> Sample {
        let amount_of_measures = Self::amount_of_measures_in_buffer(&buffer, measure_size_millis);
        Sample {
            buffer: buffer,
            amount_of_measures: amount_of_measures,
            measure_size_millis: measure_size_millis,
            time_cursor: 0,
        }
    }

    pub fn get_next_messages(&mut self, delta_time: u32) -> Vec<TypedMidiMessage> {
        let next_time_cursor = self.time_cursor + delta_time;
        let sample_size_millis = self.measure_size_millis * self.amount_of_measures;
        let mut result = Vec::new();

        self.gather_messages_in_timerange(&mut result, self.time_cursor, next_time_cursor);
        self.time_cursor = next_time_cursor % sample_size_millis;

        if next_time_cursor >= sample_size_millis {
            self.gather_messages_in_timerange(&mut result, 0, self.time_cursor);
        }

        result
    }

    fn gather_messages_in_timerange(&self, result: &mut Vec<TypedMidiMessage>, start: u32, end: u32) {
        for event in self.buffer.iter() {
            if start <= event.timestamp && event.timestamp <= end {
                result.push(event.message);
            }
        }
    }
}

pub struct Looper<'a> {
    pub state: State,
    pub next_state: Option<State>,

    pub composition: Vec<Sample>,
    pub record_buffer: Vec<TypedMidiEvent>,

    pub tempo_bpm: u32,
    pub measure_size_bpm: u32,

    pub out_port: &'a mut OutputPort,

    measure_time_cursor: u32,
}

impl<'a> Updatable for Looper<'a> {
    fn update(&mut self, delta_time: u32) {
        if self.state != State::Pause {
            let measure_size_millis = self.calc_measure_size();

            self.measure_time_cursor += delta_time;

            if self.measure_time_cursor >= measure_size_millis {
                self.measure_time_cursor %= measure_size_millis;
                self.on_measure_bar();
            }

            for sample in self.composition.iter_mut() {
                for message in sample.get_next_messages(delta_time) {
                    self.out_port.write_message(message).unwrap();
                }
            }
        }
    }
}

impl<'a> Renderable for Looper<'a> {
    fn render(&self, renderer: &mut Renderer) {
        let window_width = renderer.viewport().width();
        let window_height = renderer.viewport().height();
        let measure_size_millis = self.calc_measure_size();
        let beat_size_millis = self.calc_beat_size();

        // if self.replay_buffer.len() > 1 {
        //     let n = self.replay_buffer.len();
        //     let t0 = self.replay_buffer[0].timestamp;
        //     let tn = self.replay_buffer[n - 1].timestamp;
        //     let dt = (tn - t0) as f32;

        //     let notes = midi::events_to_notes(&self.replay_buffer);

        //     for note in notes {
        //         note.render(renderer, t0, dt);
        //     }

        //     let x = ((self.calc_abs_time_cursor() as f32) / dt * (window_width as f32 - 10.0) + 5.0) as i32;
        //     renderer.set_draw_color(Color::RGB(255, 255, 255));
        //     renderer.draw_line(Point::from((x, 0)),
        //                        Point::from((x, window_height as i32))).unwrap();

        // }

        { // Time Cursor
            let x = ((self.measure_time_cursor as f32) / measure_size_millis as f32 * (window_width as f32 - 10.0) + 5.0) as i32;
            renderer.set_draw_color(Color::RGB(255, 255, 255));
            renderer.draw_line(Point::from((x, 0)),
                               Point::from((x, window_height as i32))).unwrap();
        }

        { // Measure Beats
            for i in 0 .. self.measure_size_bpm + 1 {
                let x = (((i * beat_size_millis) as f32) / measure_size_millis as f32 * (window_width as f32 - 10.0) + 5.0) as i32;
                renderer.set_draw_color(Color::RGB(100, 100, 100));
                renderer.draw_line(Point::from((x, 0)),
                                   Point::from((x, window_height as i32))).unwrap();
            }
        }

        { // Circle
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
}

impl<'a> Looper<'a> {
    pub fn new(out_port: &'a mut OutputPort) -> Looper<'a> {
        let mut looper = Looper {
            state: State::Looping,
            next_state: None,
            composition: Vec::new(),
            record_buffer: Vec::new(),
            tempo_bpm: DEFAULT_TEMPO_BPM,
            measure_size_bpm: DEFAULT_MEASURE_SIZE_BPM,
            out_port: out_port,
            measure_time_cursor: 0,
        };
        looper.reset();
        looper
    }

    pub fn calc_beat_size(&self) -> u32 {
        (60.0 * 1000.0 / self.tempo_bpm as f32) as u32
    }

    pub fn calc_measure_size(&self) -> u32 {
        (60.0 * 1000.0 / self.tempo_bpm as f32 * self.measure_size_bpm as f32) as u32
    }

    pub fn reset(&mut self) {
        let beats = self.beat_sample();

        self.measure_time_cursor = 0;
        self.state = State::Looping;
        self.composition.clear();
        self.composition.push(beats);
        self.record_buffer.clear();
    }

    pub fn toggle_recording(&mut self) {
        match self.state {
            State::Recording => {
                self.next_state = Some(State::Looping);
            }

            State::Looping => {
                self.state = State::Recording;
                self.record_buffer.clear();
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

    pub fn on_measure_bar(&mut self) {
        let measure_size_millis = self.calc_measure_size();

        if let Some(state) = self.next_state.take() {
            self.state = state;

            match self.state {
                State::Looping => {
                    self.normalize_record_buffer();
                    self.composition.push(Sample::new(self.record_buffer.clone(),
                                                      measure_size_millis))
                },

                _ => ()
            }
        }
    }

    pub fn on_midi_event(&mut self, event: &TypedMidiEvent) {
        if let State::Recording = self.state {
            self.record_buffer.push(event.clone());
        }

        self.out_port.write_message(event.message).unwrap();
    }

    fn beat_sample(&self) -> Sample {
        let beat_size_millis = self.calc_beat_size();

        let mut buffer = Vec::new();

        for i in 0..self.measure_size_bpm {
            buffer.push(TypedMidiEvent {
                message: TypedMidiMessage::NoteOn {
                    channel: CONTROL_CHANNEL_NUMBER,
                    key: BEAT_KEY_NUMBER,
                    velocity: if i == 0 { BEAT_ACCENT_VELOCITY } else { BEAT_VELOCITY },
                },
                timestamp: i * beat_size_millis,
            });

            buffer.push(TypedMidiEvent {
                message: TypedMidiMessage::NoteOff {
                    channel: CONTROL_CHANNEL_NUMBER,
                    key: BEAT_KEY_NUMBER,
                    velocity: 0,
                },
                timestamp: i * beat_size_millis + 1,
            })
        }

        Sample::new(buffer, self.calc_measure_size())
    }

    fn normalize_record_buffer(&mut self) {
        if !self.record_buffer.is_empty() {
            let t0 = self.record_buffer[0].timestamp;

            for event in self.record_buffer.iter_mut() {
                event.timestamp -= t0;
            }
        }
    }
}
