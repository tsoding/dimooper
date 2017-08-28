use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::rect::Point;
use sdl2::keyboard::Keycode;

use screen::Screen;
use midi::*;
use config::Config;
use ui::VirtualKeyboard;
use traits::*;

pub struct KeyboardLayoutScreen {
    config: Config,
    virtual_keyboard: VirtualKeyboard,
    quit: bool,
}

impl KeyboardLayoutScreen {
    pub fn new(config: Config) -> KeyboardLayoutScreen {
        KeyboardLayoutScreen {
            virtual_keyboard: VirtualKeyboard::from_config(&config),
            config: config,
            quit: false
        }
    }
}

impl Screen<Config> for KeyboardLayoutScreen {
    // TODO: implement unbind key operation for the keyboard mode
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

    // TODO: replay midi events on the keyboard mode
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
        }
    }

    fn update(&mut self, delta_time: u32) -> Option<Config> {
        self.virtual_keyboard.update(delta_time);

        if self.quit {
            self.virtual_keyboard.to_config(&mut self.config);
            Some(self.config.clone())
        } else {
            None
        }
    }

    fn render(&self, renderer: &mut Renderer) {
        self.virtual_keyboard.render(renderer);
    }
}
