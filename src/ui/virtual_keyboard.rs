use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;

use std::collections::HashMap;

use traits::*;
use config::Config;

pub struct VirtualKeyboard {
    layout: HashMap<u64, u8>
}

// TODO: Implement VirtualKeyboard
impl VirtualKeyboard {
    pub fn from_config(config: &Config) -> VirtualKeyboard {
        VirtualKeyboard {
            layout: config.keyboard_layout.clone()
        }
    }

    pub fn to_config(&self, config: &mut Config) {
        config.keyboard_layout = self.layout.clone();
    }

    pub fn activate_binding(&mut self, keycode: &Keycode) {
        unimplemented!()
    }

    pub fn cancel_binding(&mut self) {
        unimplemented!()
    }

    pub fn bind_midicode(&mut self, midicode: u8) {
        unimplemented!()
    }
}

impl Renderable for VirtualKeyboard {
    fn render(&self, renderer: &mut Renderer) {
    }
}

impl Updatable for VirtualKeyboard {
    fn update(&mut self, delta_time: u32) {
    }
}
