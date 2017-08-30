use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use traits::*;
use hardcode::*;
use fundamental::option::*;

pub struct VirtualKey {
    keycode: Keycode,
    midicode: Option<u8>,
    active: bool,
}

const UNBOUND_COLOR: Color = Color::RGB(100, 100, 100);
const BOUND_COLOR: Color = Color::RGB(100, 200, 100);
const ACTIVE_COLOR: Color = Color::RGB(200, 200, 100);

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
    // TODO(#250): Display key names on the virtual keys
    fn render(&self, renderer: &mut Renderer) {
        self.active
            .as_option(|| ACTIVE_COLOR)
            .or(self.midicode.map(|_| BOUND_COLOR))
            .or(Some(UNBOUND_COLOR))
            .map(|color| renderer.set_draw_color(color))
            .unwrap();

        let viewport = renderer.viewport();

        renderer
            .fill_rect(Rect::new(0, 0, viewport.width(), viewport.height()))
            .expect("Cannot render a virtualkey");
    }
}
