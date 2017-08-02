use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::pixels::Color;

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
    }

    pub fn cancel_binding(&mut self) {
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
