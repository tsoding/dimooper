use std::{path, fs};
use std::io::prelude::*;

use midi::*;
use hardcode::*;
use num::integer::lcm;
use rustc_serialize::json;
use looper::Composition;
use error::Result;

use traits::{Updatable, Renderable};
use graphics_primitives::CircleRenderer;
use measure::*;
use looper::Sample;

use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;

#[derive(PartialEq)]
enum State {
    Recording,
    Looping,
    Pause,
}

pub struct Looper<NoteTracker: MidiNoteTracker> {
    state: State,
    next_state: Option<State>,

    composition: Vec<Sample>,
    record_buffer: Vec<AbsMidiEvent>,

    note_tracker: NoteTracker,

    time_cursor: u32,
    amount_of_measures: u32,

    measure: Measure,
}

impl<NoteTracker: MidiNoteTracker> Updatable for Looper<NoteTracker> {
    fn update(&mut self, delta_time: u32) {
        if self.state != State::Pause {
            let current_measure_bar = self.measure.timestamp_to_measure(self.time_cursor);
            let current_quant = self.measure.timestamp_to_quant(self.time_cursor);

            let next_time_cursor = self.time_cursor + delta_time;
            let next_measure_bar = self.measure.timestamp_to_measure(next_time_cursor);
            let next_quant = self.measure.timestamp_to_quant(next_time_cursor);

            if current_measure_bar < next_measure_bar {
                self.on_measure_bar();
            }

            if current_quant < next_quant {
                for sample in &mut self.composition {
                    // FIXME(#140): make Quants range iterable
                    let Quant(start) = current_quant;
                    let Quant(end) = next_quant;
                    for q in start + 1..end + 1 {
                        sample.replay_quant(Quant(q), &mut self.note_tracker);
                    }
                }
            }

            self.time_cursor = next_time_cursor % (self.measure.measure_size_millis() * self.amount_of_measures);
        }
    }
}

impl<NoteTracker: MidiNoteTracker> Renderable for Looper<NoteTracker> {
    fn render(&self, renderer: &mut Renderer) {
        let window_width = renderer.viewport().width();
        let window_height = renderer.viewport().height();
        let measure_size_millis = self.measure.measure_size_millis();
        let beat_size_millis = self.measure.beat_size_millis();

        for sample in &self.composition {
            sample.render(self.measure.timestamp_to_measure(self.time_cursor), renderer);
        }

        let draw_time_cursor = |time_cursor: u32, renderer: &mut Renderer| {
            let x = ((time_cursor as f32) /
                     measure_size_millis as f32 *
                     (window_width as f32 - 10.0) + 5.0) as i32;
            renderer.draw_line(Point::from((x, 0)),
                               Point::from((x, window_height as i32))).unwrap();
        };

        // FIXME(#148): Separate Looper::render into several functions.
        // If you need separate comments like this, you need separate
        // functions

        // Time Cursor
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        draw_time_cursor(self.time_cursor % measure_size_millis, renderer);

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

impl<NoteTracker: MidiNoteTracker> Looper<NoteTracker> {
    pub fn new(note_tracker: NoteTracker) -> Looper<NoteTracker> {
        let mut looper = Looper {
            state: State::Looping,
            next_state: None,
            composition: Vec::new(),
            record_buffer: Vec::new(),
            note_tracker: note_tracker,
            amount_of_measures: 1,
            time_cursor: 0,
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

        self.amount_of_measures = 1;
        self.time_cursor = self.amount_of_measures * self.measure.measure_size_millis() - 1;

        self.note_tracker.close_opened_notes();
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
                self.note_tracker.close_opened_notes();
            },
            State::Pause => self.state = State::Looping,
            _ => (),
        }
    }

    pub fn undo_last_recording(&mut self) {
        if let State::Recording = self.state {
            self.record_buffer.clear();
        } else if self.composition.len() > 1  {
            self.composition.pop();
            self.amount_of_measures = 1;
            for sample in &self.composition {
                self.amount_of_measures = lcm(self.amount_of_measures,
                                              sample.amount_of_measures);
            }
            self.note_tracker.close_opened_notes();
        }
    }

    pub fn on_measure_bar(&mut self) {
        if let Some(state) = self.next_state.take() {
            self.state = state;

            if let State::Looping = self.state {
                let current_measure = self.measure.timestamp_to_measure(self.time_cursor);
                self.normalize_record_buffer();
                // FIXME(#164): Separate Sample::amount_of_measures_in_buffer from Sample
                let sample_amount_of_measures = self.measure.amount_of_measures_in_buffer(&self.record_buffer);
                self.amount_of_measures = lcm(self.amount_of_measures, sample_amount_of_measures);
                let sample = Sample::new(&self.record_buffer, &self.measure, self.amount_of_measures - current_measure - 1);
                self.composition.push(sample);
            }
        }
    }

    pub fn on_midi_event(&mut self, event: &AbsMidiEvent) {
        if let State::Recording = self.state {
            self.record_buffer.push(*event);
        }

        self.note_tracker.feed(event.message).unwrap();
    }

    pub fn update_tempo_bpm(&mut self, tempo_bpm: u32) {
        let new_measure = Measure { tempo_bpm: tempo_bpm, .. self.measure };

        // FIXME(#150): Improve time cursor scaling
        self.time_cursor =
            self.measure.scale_time_cursor(&new_measure,
                                           self.amount_of_measures,
                                           self.time_cursor % (self.amount_of_measures * self.measure.measure_size_millis()));

        self.measure = new_measure;
    }

    pub fn load_state_from_file(&mut self, path: &path::Path) -> Result<()> {
        let mut file = try!(fs::File::open(path));
        let mut serialized_composition = String::new();
        try!(file.read_to_string(&mut serialized_composition));
        let composition: Composition = try!(json::decode(&serialized_composition));

        self.note_tracker.close_opened_notes();
        self.composition = composition.samples;
        self.measure = composition.measure;
        self.time_cursor = 0;

        // TODO(#186): Extract recalculation of the amount of measures.
        //
        // Right know Looper has a lot of duplicate code for
        // recalculating amount of measures of the composition. We
        // need to centralize that somehow.
        self.amount_of_measures = 1;
        for sample in &self.composition {
            self.amount_of_measures = lcm(self.amount_of_measures,
                                          sample.amount_of_measures);
        }
        self.time_cursor = self.amount_of_measures * self.measure.measure_size_millis() - 1;

        Ok(())
    }

    pub fn save_state_to_file(&self, path: &path::Path) -> Result<()> {
        let composition: Composition = Composition::new(self.measure.clone(),
                                                        self.composition.clone());

        let serialized_composition: String = try!(json::encode(&composition));
        let mut file = try!(fs::File::create(path));
        try!(file.write_all(serialized_composition.as_bytes()));

        Ok(())
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

        Sample::new(&buffer, &self.measure, 0)
    }

    fn normalize_record_buffer(&mut self) {
        if !self.record_buffer.is_empty() {
            let t0 = self.record_buffer[0].timestamp;

            for event in &mut self.record_buffer {
                event.timestamp -= t0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Looper;
    use pm::types::Result;
    use midi::{TypedMidiMessage, MidiSink, MidiNoteTracker};

    pub struct DummyMidiNoteTracker;

    impl MidiNoteTracker for DummyMidiNoteTracker {
        fn close_opened_notes(&mut self) {}
    }

    impl MidiSink for DummyMidiNoteTracker {
        fn feed(&mut self, _: TypedMidiMessage) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_looper_initial_time_cursor() {
        let looper = Looper::new(DummyMidiNoteTracker);
        assert_eq!(looper.time_cursor, looper.amount_of_measures * looper.measure.measure_size_millis() - 1);
    }
}
