use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;

use num::*;

use std::collections::HashMap;

use traits::*;
use config::Config;

use ui::VirtualKey;

pub struct VirtualKeyboard {
    virtual_keys: HashMap<Keycode, VirtualKey>,
    active_key: Option<Keycode>,
}

impl VirtualKeyboard {
    pub fn from_config(config: &Config) -> VirtualKeyboard {
        VirtualKeyboard {
            virtual_keys: {
                let mut virtual_keys = HashMap::new();

                for (i, row) in ["qwertyuiop", "asdfghjkl", "zxcvbnm"].iter().enumerate() {
                    for (j, key) in row.chars().enumerate() {
                        let keycode = Keycode::from_name(key.to_string().as_str()).unwrap();
                        let code = keycode.to_u64().unwrap();
                        let midicode = config.keyboard_layout.get(&code).cloned();
                        virtual_keys.insert(keycode, VirtualKey::new((i, j), keycode, midicode));
                    }
                }

                virtual_keys
            },
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
        for (_, virtual_key) in self.virtual_keys.iter() {
            virtual_key.render(renderer);
        }
    }
}

impl Updatable for VirtualKeyboard {
    fn update(&mut self, delta_time: u32) {
        for (_, virtual_key) in self.virtual_keys.iter_mut() {
            virtual_key.update(delta_time);
        }
    }
}
