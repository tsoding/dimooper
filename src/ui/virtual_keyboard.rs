use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::pixels::Color;

use num::*;

use std::collections::HashMap;

use traits::*;
use config::Config;

use ui::VirtualKey;

pub struct VirtualKeyboard {
    virtual_keys: HashMap<Keycode, VirtualKey>,
    active_key: Option<Keycode>,
}

// TODO(#242): Implement VirtualKeyboard
impl VirtualKeyboard {
    pub fn from_config(config: &Config) -> VirtualKeyboard {
        VirtualKeyboard {
            virtual_keys: ["qwertyuiop", "asdfghjkl", "zxcvbnm"]
               .iter()
               .flat_map(|row| {
                   row.chars().map(|key| {
                       let keycode = Keycode::from_name(key.to_string().as_str()).unwrap();
                       let code = keycode.to_u64().unwrap();
                       let midicode = config.keyboard_layout.get(&code).cloned();
                       (keycode, VirtualKey::new(&keycode, &midicode))
                   })
               })
              .collect(),
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
    }
}

impl Renderable for VirtualKeyboard {
    fn render(&self, renderer: &mut Renderer) {
        let window_width = renderer.viewport().width() as i32;
        let window_height = renderer.viewport().height() as i32;

        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.draw_line(Point::from((0, 0)), Point::from((window_width, window_height)));
        renderer.draw_line(Point::from((window_width, 0)), Point::from((0, window_height)));
    }
}

impl Updatable for VirtualKeyboard {
    fn update(&mut self, delta_time: u32) {
    }
}
