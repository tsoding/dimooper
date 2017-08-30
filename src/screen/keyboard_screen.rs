use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use screen::Screen;
use midi::*;
use config::Config;
use ui::VirtualKeyboard;
use traits::*;

pub struct KeyboardScreen<NoteTracker: MidiNoteTracker> {
    config: Config,
    virtual_keyboard: VirtualKeyboard,
    note_tracker: NoteTracker,
    quit: bool,
}

impl<NoteTracker: MidiNoteTracker> KeyboardScreen<NoteTracker> {
    pub fn new(note_tracker: NoteTracker, config: Config) -> Self {
        Self {
            virtual_keyboard: VirtualKeyboard::from_config(&config),
            config: config,
            quit: false,
            note_tracker: note_tracker,
        }
    }
}

impl<NoteTracker: MidiNoteTracker> Screen<Config> for KeyboardScreen<NoteTracker> {
    // TODO(#247): Implement unbind key operation for the keyboard mode
    fn handle_sdl_events(&mut self, events: &[Event]) {
        for event in events {
            match *event {
                Event::Quit { .. } => {
                    self.quit = true
                },

                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => {
                    self.virtual_keyboard.cancel_binding()
                }

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.virtual_keyboard.activate_binding(&keycode);
                },

                _ => {}
            }
        }
    }

    fn handle_midi_events(&mut self, events: &[AbsMidiEvent]) {
        for event in events {
            match *event {
                AbsMidiEvent {
                    message: TypedMidiMessage::NoteOn {
                        key: midicode,
                        ..
                    },
                    ..
                } => {
                    self.virtual_keyboard.bind_midicode(midicode)
                },

                _ => {}
            }

            self.note_tracker.feed(event.message);
        }
    }

    fn update(&mut self, _: u32) -> Option<Config> {
        if self.quit {
            self.note_tracker.close_opened_notes();
            self.virtual_keyboard.to_config(&mut self.config);
            Some(self.config.clone())
        } else {
            None
        }
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.set_draw_color(Color::RGB(24, 24, 24));
        renderer.clear();
        self.virtual_keyboard.render(renderer);
    }
}
