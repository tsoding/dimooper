use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use traits::*;
use hardcode::*;

pub struct VirtualKey {
    keycode: Keycode,
    midicode: Option<u8>,
    active: bool,
}

impl VirtualKey {
    pub fn new(keycode: Keycode,
               midicode: Option<u8>) -> VirtualKey {
        VirtualKey {
            keycode: keycode,
            midicode: midicode,
            active: false,
        }
    }

    pub fn activate_binding(&mut self) {
        self.active = true;
    }

    pub fn cancel_binding(&mut self) {
        self.active = false;
    }

    pub fn bind_midicode(&mut self, midicode: u8) {
        self.active = false;
        self.midicode = Some(midicode);
    }

    pub fn as_midicode(&self) -> Option<u8> {
        self.midicode
    }
}

impl Renderable for VirtualKey {
    fn render(&self, renderer: &mut Renderer) {
        if (self.active) {
            renderer.set_draw_color(Color::RGB(200, 200, 100));
        } else {
            if (self.midicode.is_some()) {
                renderer.set_draw_color(Color::RGB(100, 200, 100))
            } else {
                renderer.set_draw_color(Color::RGB(100, 100, 100));
            }
        }
        renderer.fill_rect(Rect::new(0, 0, VIRTUAL_KEY_WIDTH, VIRTUAL_KEY_HEIGHT));
    }
}

impl Updatable for VirtualKey {
    fn update(&mut self, delta_time: u32) {
    }
}
