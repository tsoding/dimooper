use midi;
use midi::{AbsMidiEvent, TypedMidiMessage};
use config::*;
use num::integer::lcm;
use midi_adapter::MidiAdapter;

use traits::{Updatable, Renderable};
use graphicsprimitives::CircleRenderer;
use measure::{Measure, Quant};

use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub mod sample;

use self::sample::{Sample, QuantMidiEvent};

#[derive(PartialEq)]
pub enum State {
    Recording,
    Looping,
    Pause,
}

pub struct Looper {
    pub state: State,
    pub next_state: Option<State>,

    pub composition: Vec<Sample>,
    pub record_buffer: Vec<AbsMidiEvent>,


    pub midi_adapter: MidiAdapter,

    measure_time_cursor: u32,
    measure_cursor: u32,
    amount_of_measures: u32,

    pub measure: Measure,
}

impl Updatable for Looper {
    fn update(&mut self, delta_time: u32) {
        if self.state != State::Pause {
            let measure_size_millis = self.measure.measure_size_millis();

            self.measure_time_cursor += delta_time;

            if self.measure_time_cursor >= measure_size_millis {
                self.measure_time_cursor %= measure_size_millis;
                self.measure_cursor = (self.measure_cursor + 1) % self.amount_of_measures;
                self.on_measure_bar();
            }

            for sample in self.composition.iter_mut() {
                for message in sample.get_next_messages(delta_time) {
                    self.midi_adapter.write_message(message).unwrap();
                }
            }
        }
    }
}

impl Renderable for Looper {
    fn render(&self, renderer: &mut Renderer) {
        let window_width = renderer.viewport().width();
        let window_height = renderer.viewport().height();
        let measure_size_millis = self.measure.measure_size_millis();
        let beat_size_millis = self.measure.beat_size_millis();

        for sample in &self.composition {
            sample.render(renderer);
        }

        let draw_time_cursor = |time_cursor: u32, renderer: &mut Renderer| {
            let x = ((time_cursor as f32) /
                     measure_size_millis as f32 *
                     (window_width as f32 - 10.0) + 5.0) as i32;
            renderer.draw_line(Point::from((x, 0)),
                               Point::from((x, window_height as i32))).unwrap();
        };

        // Time Cursor
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        draw_time_cursor(self.measure_time_cursor, renderer);

        // Measure Beats
        for i in 0 .. self.measure.measure_size_bpm {
            renderer.set_draw_color(Color::RGB(50, 50, 50));
            draw_time_cursor(i * beat_size_millis, renderer);
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

impl Looper {
    pub fn new(midi_adapter: MidiAdapter) -> Looper {
        let mut looper = Looper {
            state: State::Looping,
            next_state: None,
            composition: Vec::new(),
            record_buffer: Vec::new(),
            midi_adapter: midi_adapter,
            measure_time_cursor: 0,
            measure_cursor: 0,
            amount_of_measures: 1,
            measure: Measure {
                tempo_bpm: DEFAULT_TEMPO_BPM,
                measure_size_bpm: DEFAULT_MEASURE_SIZE_BPM,
                quantation_level: DEFAULT_QUANTATION_LEVEL,
            },
        };
        looper.reset();
        looper
    }

    pub fn reset(&mut self) {
        let beats = self.make_metronome();

        self.state = State::Looping;
        self.composition.clear();
        self.composition.push(beats);
        self.record_buffer.clear();

        self.measure_time_cursor = 0;
        self.measure_cursor = 0;
        self.amount_of_measures = 1;

        self.midi_adapter.close_notes();
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
            State::Looping => {
                self.state = State::Pause;
                self.midi_adapter.close_notes();
            },
            State::Pause => self.state = State::Looping,
            _ => (),
        }
    }

    pub fn undo_last_recording(&mut self) {
        if let State::Recording = self.state {
            self.record_buffer.clear();
        } else {
            if self.composition.len() > 1 {
                self.composition.pop();
                self.amount_of_measures = 1;
                for sample in &self.composition {
                    self.amount_of_measures = lcm(self.amount_of_measures,
                                                  sample.amount_of_measures);
                }
                self.midi_adapter.close_notes();
            }
        }
    }

    pub fn on_measure_bar(&mut self) {
        if let Some(state) = self.next_state.take() {
            self.state = state;

            match self.state {
                State::Looping => {
                    self.normalize_record_buffer();
                    let sample = Sample::new(&self.record_buffer, &self.measure);
                    self.amount_of_measures = lcm(self.amount_of_measures, sample.amount_of_measures);
                    self.composition.push(sample);
                },

                _ => ()
            }
        }
    }

    pub fn on_midi_event(&mut self, event: &AbsMidiEvent) {
        if let State::Recording = self.state {
            self.record_buffer.push(event.clone());
        }

        self.midi_adapter.write_message(event.message).unwrap();
    }

    pub fn update_tempo_bpm(&mut self, tempo_bpm: u32) {
        let new_measure = Measure { tempo_bpm: tempo_bpm, .. self.measure };

        self.measure_time_cursor =
            self.measure.scale_time_cursor(&new_measure,
                                           self.amount_of_measures,
                                           self.measure_time_cursor);

        for sample in self.composition.iter_mut() {
            sample.update_measure(&new_measure)
        }

        self.measure = new_measure;
    }

    fn make_metronome(&self) -> Sample {
        let beat_size_millis = self.measure.beat_size_millis();

        let mut buffer = Vec::new();

        for i in 0..self.measure.measure_size_bpm {
            buffer.push(AbsMidiEvent {
                message: TypedMidiMessage::NoteOn {
                    channel: CONTROL_CHANNEL_NUMBER,
                    key: BEAT_KEY_NUMBER,
                    velocity: if i == 0 { BEAT_ACCENT_VELOCITY } else { BEAT_VELOCITY },
                },
                timestamp: i * beat_size_millis,
            });

            buffer.push(AbsMidiEvent {
                message: TypedMidiMessage::NoteOff {
                    channel: CONTROL_CHANNEL_NUMBER,
                    key: BEAT_KEY_NUMBER,
                    velocity: 0,
                },
                timestamp: i * beat_size_millis + 1,
            })
        }

        Sample::new(&buffer, &self.measure)
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
