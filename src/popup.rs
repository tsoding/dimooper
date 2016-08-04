use std::cmp;

use sdl2::render::Renderer;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2_ttf::Font;

use renderable::Renderable;
use updatable::Updatable;

use config::POPUP_FADEOUT_TIME;

pub struct Popup {
    text: String,
    font: Font,
    countdown: u32,
}

impl Popup {
    pub fn new(label_text: &str, font: Font) -> Popup {
        Popup {
            text: String::from(label_text),
            font: font,
            countdown: 0,
        }
    }

    /// Sets the text of popup and brings opacity to maximum.
    ///
    /// Once the popup is bumped it's gonna become visible and slowly
    /// fade out until it's bumped again.
    pub fn bump(&mut self, label_text: &str) {
        self.text = String::from(label_text);
        self.countdown = POPUP_FADEOUT_TIME;
    }
}

impl Renderable for Popup {
    fn render(&self, renderer: &mut Renderer) {
        if self.countdown > 0 {
            let window_width = renderer.viewport().width();
            let window_height = renderer.viewport().height();

            let popup_surface = self.font.render(self.text.as_str()).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            let mut texture = renderer.create_texture_from_surface(popup_surface).unwrap();

            texture.set_alpha_mod((255 as f32 / POPUP_FADEOUT_TIME as f32 * self.countdown as f32) as u8);

            let TextureQuery { width, height, .. } = texture.query();

            renderer.copy(&mut texture, None, Some(Rect::new(10, 10, width, height)));
        }
    }
}

impl Updatable for Popup {
    fn update(&mut self, delta_time: u32) {
        self.countdown -= cmp::min(self.countdown, delta_time);
    }
}
