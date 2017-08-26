use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use num::*;

use std::collections::HashMap;

use traits::*;
use config::Config;
use hardcode::*;

use ui::VirtualKey;

const KEYBOARD_LAYOUT: [&str; 3] = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];

pub struct VirtualKeyboard {
    virtual_keys: HashMap<Keycode, VirtualKey>,
    active_key: Option<Keycode>,
}

impl VirtualKeyboard {
    pub fn from_config(config: &Config) -> VirtualKeyboard {
        let mut virtual_keys = HashMap::new();

        for row in KEYBOARD_LAYOUT.iter() {
            for key in row.chars() {
                let keycode = Keycode::from_name(key.to_string().as_str()).unwrap();
                let code = keycode.to_u64().unwrap();
                let midicode = config.keyboard_layout.get(&code).cloned();
                virtual_keys.insert(keycode, VirtualKey::new(keycode, midicode));
            }
        }

        VirtualKeyboard {
            virtual_keys: virtual_keys,
            active_key: None,
        }
    }

    pub fn to_config(&self, config: &mut Config) {
        config.keyboard_layout =
            self.virtual_keys.iter().filter_map(|(keycode, virtual_key)| {
                virtual_key
                    .as_midicode()
                    .and_then(|midicode| keycode.to_u64().map(|keycode| (keycode, midicode)))
            }).collect();
    }

    pub fn activate_binding(&mut self, keycode: &Keycode) {
        self.cancel_binding();
        self.active_key = self.virtual_keys.get_mut(&keycode).map(|virtual_key| {
            virtual_key.activate_binding();
            keycode.clone()
        });
    }

    pub fn cancel_binding(&mut self) {
        self.active_key = self.active_key
            .and_then(|keycode|{
                self.virtual_keys.get_mut(&keycode)
            })
            .and_then(|virtual_key| {
                virtual_key.cancel_binding();
                None
            });
    }

    pub fn bind_midicode(&mut self, midicode: u8) {
        self.active_key = self.active_key
            .and_then(|keycode| {
                self.virtual_keys.get_mut(&keycode)
            })
            .and_then(|virtual_key| {
                virtual_key.bind_midicode(midicode);
                None
            });
    }
}

impl Renderable for VirtualKeyboard {
    fn render(&self, renderer: &mut Renderer) {
        let old_viewport = renderer.viewport();

        for (i, row) in KEYBOARD_LAYOUT.iter().enumerate() {
            for (j, key) in row.chars().enumerate() {
                Keycode::from_name(key.to_string().as_str())
                    .and_then(|keycode| self.virtual_keys.get(&keycode))
                    .map(|virtual_key| {
                        renderer.set_viewport(Some(Rect::new((j as i32) * (VIRTUAL_KEY_WIDTH as i32 + VIRTUAL_KEY_SPACING),
                                                             (i as i32) * (VIRTUAL_KEY_HEIGHT as i32 + VIRTUAL_KEY_SPACING),
                                                             VIRTUAL_KEY_WIDTH,
                                                             VIRTUAL_KEY_HEIGHT)));
                        virtual_key.render(renderer);
                    });
            }
        }

        renderer.set_viewport(Some(old_viewport));
    }
}

impl Updatable for VirtualKeyboard {
    fn update(&mut self, delta_time: u32) {
        for (_, virtual_key) in self.virtual_keys.iter_mut() {
            virtual_key.update(delta_time);
        }
    }
}
