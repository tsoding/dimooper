use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use midi::*;
use screen::{StateId, Screen};
use ui::Popup;
use looper::Looper;
use hardcode::*;
use std::path::Path;
use traits::*;

pub struct LooperScreen<NoteTracker: MidiNoteTracker> {
    looper: Looper<NoteTracker>,
    bpm_popup: Popup,
    next_state: StateId
}

impl<NoteTracker: MidiNoteTracker> LooperScreen<NoteTracker> {
    pub fn new(looper: Looper<NoteTracker>, bpm_popup: Popup) -> LooperScreen<NoteTracker> {
        LooperScreen {
            looper: looper,
            bpm_popup: bpm_popup,
            next_state: StateId::MainLooper
        }
    }
}

impl<NoteTracker: MidiNoteTracker> Screen for LooperScreen<NoteTracker> {
    fn handle_sdl_events(&mut self, events: &[Event]) {
        for event in events {
            match *event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.next_state = StateId::Quit;
                    self.looper.reset();
                }

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    self.looper.toggle_recording();
                }

                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    self.looper.reset();
                }

                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    self.looper.undo_last_recording();
                }

                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    self.looper.toggle_pause();
                }

                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    match self.looper.save_state_to_file(Path::new(STATE_FILE_PATH)) {
                        Ok(_) => println!("Saved looper state to {}", STATE_FILE_PATH),
                        Err(e) => println!("[ERROR] {}", e),
                    }
                }

                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    match self.looper.load_state_from_file(Path::new(STATE_FILE_PATH)) {
                        Ok(_) => println!("Loaded looper state from {}", STATE_FILE_PATH),
                        Err(e) => println!("[ERROR] {}", e),
                    }
                }

                _ => {}
            }
        }
    }

    fn handle_midi_events(&mut self, events: &[AbsMidiEvent]) {
        for event in events {
            // FIXME(#149): Extract MIDI logging into a separate entity
            println!("{:?}", event.message);

            match *event {
                AbsMidiEvent {
                    message: TypedMidiMessage::ControlChange {
                        number: TEMPO_CHANGE_CONTROL_NUMBER,
                        value,
                        ..
                    },
                    ..
                } => {
                    let bpm = value as u32 + 90;
                    self.looper.update_tempo_bpm(bpm);
                    self.bpm_popup.bump(format!("{:03}", bpm).as_str());
                },

                AbsMidiEvent {
                    message: TypedMidiMessage::NoteOn {
                        key: CONTROL_KEY_NUMBER,
                        channel: CONTROL_CHANNEL_NUMBER,
                        ..
                    },
                    ..
                } => {
                    self.looper.toggle_recording();
                },

                AbsMidiEvent {
                    message: TypedMidiMessage::NoteOff {
                        key: CONTROL_KEY_NUMBER,
                        channel: CONTROL_CHANNEL_NUMBER,
                        ..
                    },
                    ..
                } => {},

                _ => self.looper.on_midi_event(&event),

            }
        }
    }

    fn update(&mut self, delta_time: u32) -> StateId {
        self.looper.update(delta_time);
        self.bpm_popup.update(delta_time);
        self.next_state
    }

    fn render(&self, renderer: &mut Renderer) {
        self.looper.render(renderer);
        self.bpm_popup.render(renderer);
    }
}
