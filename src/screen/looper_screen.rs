use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::pixels::Color;

use std::path::Path;

use midi::*;
use screen::Screen;
use ui::Popup;
use looper::Looper;
use hardcode::*;
use traits::*;
use path;
use screen::KeyboardLayout;
use config::Config;

pub struct LooperScreen<NoteTracker: MidiNoteTracker> {
    timestamp: u32,
    looper: Looper<NoteTracker>,
    bpm_popup: Popup,
    quit: bool,
    keyboard_layout: KeyboardLayout,
}

impl<NoteTracker: MidiNoteTracker> LooperScreen<NoteTracker> {
    pub fn new(looper: Looper<NoteTracker>,
               bpm_popup: Popup,
               config: &Config) -> LooperScreen<NoteTracker> {
        LooperScreen {
            looper: looper,
            bpm_popup: bpm_popup,
            quit: false,
            keyboard_layout: KeyboardLayout::from_config(config),
            timestamp: 0,
        }
    }

}

impl<NoteTracker: MidiNoteTracker> Screen<()> for LooperScreen<NoteTracker> {
    fn handle_sdl_events(&mut self, events: &[Event]) {
        for event in events {
            // TODO: Hardcoded key bindings in looper mode collide with the key bound via the keyboard mode
            match *event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.quit = true;
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
                    let state_file_path = Path::new(STATE_FILE_PATH);
                    let absolute_path = path::display_absolute_path(state_file_path);
                    match self.looper.save_state_to_file(state_file_path) {
                        Ok(_) => println!("Saved looper state to {}", absolute_path.display()),
                        Err(e) => println!("[ERROR] Could not save state to {}. Reason: {}",
                                           absolute_path.display(),
                                           e),
                    }
                }

                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    let state_file_path = Path::new(STATE_FILE_PATH);
                    let absolute_path = path::display_absolute_path(state_file_path);
                    match self.looper.load_state_from_file(state_file_path) {
                        Ok(_) => println!("Loaded looper state from {}", absolute_path.display()),
                        Err(e) => println!("[ERROR] Could not load state from {}. Reason: {}",
                                           absolute_path.display(),
                                           e),
                    }
                }

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.keyboard_layout.key_down::<NoteTracker>(&mut self.looper,
                                                                 &keycode,
                                                                 self.timestamp);
                }

                Event::KeyUp { keycode: Some(keycode), .. } => {
                    self.keyboard_layout.key_up::<NoteTracker>(&mut self.looper,
                                                               &keycode,
                                                               self.timestamp);
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

    fn update(&mut self, delta_time: u32) -> Option<()> {
        // TODO(#217): calculate current timestamp with PortMidi mechanisms
        self.timestamp += delta_time;
        self.looper.update(delta_time);
        self.bpm_popup.update(delta_time);

        if self.quit {
            Some({})
        } else {
            None
        }
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        self.looper.render(renderer);
        self.bpm_popup.render(renderer);
    }
}
