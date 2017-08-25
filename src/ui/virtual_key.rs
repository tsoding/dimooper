use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;

use traits::*;

pub struct VirtualKey {
    keycode: Keycode,
    midicode: Option<u8>,
    position: (usize, usize),
}

impl VirtualKey {
    pub fn new(position: (usize, usize),
               keycode: Keycode,
               midicode: Option<u8>) -> VirtualKey {
        VirtualKey {
            keycode: keycode,
            midicode: midicode,
            position: position,
        }
    }

    pub fn activate_binding(&mut self) {
    }

    pub fn cancel_binding(&mut self) {
    }

    pub fn bind_midicode(&mut self, midicode: u8) {
    }

    pub fn as_midicode(&self) -> Option<u8> {
        self.midicode
    }
}

impl Renderable for VirtualKey {
    fn render(&self, renderer: &mut Renderer) {
    }
}

impl Updatable for VirtualKey {
    fn update(&mut self, delta_time: u32) {
    }
}
