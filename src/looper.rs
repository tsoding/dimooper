use midi;
use midi::{TypedMidiEvent, TypedMidiMessage};
use config::*;
use num::integer::lcm;
use midi_adapter::MidiAdapter;

use updatable::Updatable;
use renderable::Renderable;
use graphicsprimitives::CircleRenderer;
use measure::Measure;

use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;

#[derive(PartialEq)]
pub enum State {
    Recording,
    Looping,
    Pause,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuantMidiEvent {
    pub message: TypedMidiMessage,
    pub quant: u32,
}

pub struct Sample {
    pub buffer: Vec<QuantMidiEvent>,
    amount_of_measures: u32,
    time_cursor: u32,
}

impl Sample {

    fn amount_of_measures_in_buffer(buffer: &[TypedMidiEvent], measure: &Measure) -> u32 {
        let n = buffer.len();

        if n > 0 {
            (buffer[n - 1].timestamp - buffer[0].timestamp + measure.measure_size_millis()) / measure.measure_size_millis()
        } else {
            1
        }
    }

    pub fn new(buffer: &[TypedMidiEvent], measure: &Measure) -> Sample {
        let amount_of_measures = Self::amount_of_measures_in_buffer(&buffer, &measure);

        let quant_buffer = {
            let mut result = Vec::new();

            for event in buffer {
                result.push(QuantMidiEvent {
                    message: event.message,
                    quant: measure.timestamp_to_quant(event.timestamp),
                })
            }

            result
        };

        Sample {
            buffer: quant_buffer,
            amount_of_measures: amount_of_measures,
            time_cursor: 0,
        }
    }

    pub fn get_next_messages(&mut self, delta_time: u32, measure: &Measure) -> Vec<TypedMidiMessage> {
        let next_time_cursor = self.time_cursor + delta_time;
        let sample_size_millis = measure.measure_size_millis() * self.amount_of_measures;
        let mut result = Vec::new();

        self.gather_messages_in_timerange(measure, &mut result, self.time_cursor, next_time_cursor);
        self.time_cursor = next_time_cursor % sample_size_millis;

        if next_time_cursor >= sample_size_millis {
            self.gather_messages_in_timerange(measure, &mut result, 0, self.time_cursor);
        }

        result
    }

    fn gather_messages_in_timerange(&self, measure: &Measure, result: &mut Vec<TypedMidiMessage>, start: u32, end: u32) {
        for event in self.buffer.iter() {
            let timestamp = measure.quant_to_timestamp(event.quant);
            if start <= timestamp && timestamp <= end {
                result.push(event.message);
            }
        }
    }
}

pub struct Looper {
    pub state: State,
    pub next_state: Option<State>,

    pub composition: Vec<Sample>,
    pub record_buffer: Vec<TypedMidiEvent>,


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
                for message in sample.get_next_messages(delta_time, &self.measure) {
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

        let render_buffer = {
            let mut result = Vec::new();

            for sample in self.composition.iter() {
                let repeat_count = self.amount_of_measures / sample.amount_of_measures;
                for i in 0..repeat_count {
                    for event in sample.buffer.iter() {
                        result.push(TypedMidiEvent {
                            timestamp: self.measure.quant_to_timestamp(event.quant) +
                                sample.amount_of_measures * measure_size_millis * i,
                            message: event.message,
                        })
                    }
                }
            }

            result
        };

        let dt = (measure_size_millis * self.amount_of_measures) as f32;

        let notes = midi::events_to_notes(&render_buffer);

        for note in notes {
            note.render(renderer, dt);
        }

        let x = (((measure_size_millis * self.measure_cursor + self.measure_time_cursor) as f32) / dt *
                 (window_width as f32 - 10.0) + 5.0) as i32;
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.draw_line(Point::from((x, 0)),
                           Point::from((x, window_height as i32))).unwrap();


        { // Time Cursor
            let x = (((measure_size_millis * self.measure_cursor + self.measure_time_cursor) as f32) /
                     (measure_size_millis * self.amount_of_measures) as f32 *
                     (window_width as f32 - 10.0) + 5.0) as i32;
            renderer.set_draw_color(Color::RGB(255, 255, 255));
            renderer.draw_line(Point::from((x, 0)),
                               Point::from((x, window_height as i32))).unwrap();
        }

        { // Measure Beats
            for i in 0 .. self.measure.measure_size_bpm() * self.amount_of_measures {
                let x = (((i * beat_size_millis) as f32) / (measure_size_millis * self.amount_of_measures) as f32 *
                         (window_width as f32 - 10.0) + 5.0) as i32;
                renderer.set_draw_color(Color::RGB(50, 50, 50));
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
            measure: Measure::new(DEFAULT_TEMPO_BPM,
                                  DEFAULT_MEASURE_SIZE_BPM,
                                  DEFAULT_QUANTATION_LEVEL),
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

    pub fn on_midi_event(&mut self, event: &TypedMidiEvent) {
        if let State::Recording = self.state {
            self.record_buffer.push(event.clone());
        }

        self.midi_adapter.write_message(event.message).unwrap();
    }

    pub fn update_tempo_bpm(&mut self, tempo_bpm: u32) {
        self.measure.update_tempo_bpm(tempo_bpm);
    }

    fn make_metronome(&self) -> Sample {
        let beat_size_millis = self.measure.beat_size_millis();

        let mut buffer = Vec::new();

        for i in 0..self.measure.measure_size_bpm() {
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
