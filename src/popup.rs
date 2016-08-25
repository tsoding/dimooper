use std::cmp;

use sdl2::render::{Renderer, TextureQuery, Texture};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2_ttf::Font;

use traits::{Updatable, Renderable};

use config::{POPUP_FADEOUT_TIME, POPUP_STAY_TIME};

pub struct Popup {
    text: String,
    font: Font,
    countdown: u32,
}

impl Popup {
    fn calculate_alpha(&self) -> u8 {
        let raw_alpha = 255 as f32 / POPUP_FADEOUT_TIME as f32 * self.countdown as f32;
        cmp::min(255, raw_alpha as u32) as u8
    }

    fn make_text_texture(&self, renderer: &mut Renderer) -> Texture {
        let surface = self.font.render(self.text.as_str()).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
        let mut texture = renderer.create_texture_from_surface(surface).unwrap();
        texture.set_alpha_mod(self.calculate_alpha());
        texture
    }

    fn make_texture_rect(&self, window_width: u32, texture_query: TextureQuery) -> Rect {
        let TextureQuery { width, height, .. } = texture_query;

        let label_width = (window_width as f32 / 3.0) as u32;
        let label_height = (label_width as f32 / width as f32 * height as f32) as u32;

        Rect::new(label_width as i32, label_height as i32,
                  label_width, label_height)
    }

    pub fn new(font: Font) -> Popup {
        Popup {
            text: String::from(""),
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
        self.countdown = POPUP_FADEOUT_TIME + POPUP_STAY_TIME;
    }
}

impl Renderable for Popup {
    fn render(&self, renderer: &mut Renderer) {
        if self.countdown > 0 {
            let texture = self.make_text_texture(renderer);
            let texture_rect = self.make_texture_rect(renderer.viewport().width(),
                                                      texture.query());
            renderer.copy(&texture, None, Some(texture_rect));
        }
    }
}

impl Updatable for Popup {
    fn update(&mut self, delta_time: u32) {
        self.countdown -= cmp::min(self.countdown, delta_time);
    }
}

#[cfg(test)]
mod tests {
    use super::Popup;
    use traits::Updatable;
    use std::path::Path;

    use sdl2::rect::Rect;
    use sdl2::pixels::PixelFormatEnum;
    use sdl2::render::{TextureQuery, TextureAccess};

    use sdl2_ttf;
    use sdl2_ttf::Font;
    use config::{
        TTF_FONT_PATH,
        POPUP_STAY_TIME,
        POPUP_FADEOUT_TIME,
    };

    fn load_default_font() -> Font {
        let ttf_context = sdl2_ttf::init().unwrap();
        ttf_context.load_font(Path::new(TTF_FONT_PATH), 50).unwrap()
    }

    #[test]
    #[ignore]
    fn test_bump_alpha() {
        let mut popup = Popup::new(load_default_font());
        popup.bump("khooy");

        let initial_alpha = popup.calculate_alpha();
        assert_eq!(255, initial_alpha);
        assert_eq!("khooy", popup.text);

        popup.update(POPUP_STAY_TIME / 2);
        let stable_alpha = popup.calculate_alpha();
        assert_eq!(initial_alpha, stable_alpha);

        popup.update(POPUP_STAY_TIME / 2 + POPUP_FADEOUT_TIME / 2);
        let fadeout_alpha = popup.calculate_alpha();
        assert!(initial_alpha > fadeout_alpha);
    }

    #[test]
    #[ignore]
    fn test_make_texture_rect() {
        let popup = Popup::new(load_default_font());

        let rect = popup.make_texture_rect(1002, TextureQuery {
            format: PixelFormatEnum::Unknown,
            access: TextureAccess::Static,
            width: 334,
            height: 200,
        });

        assert_eq!(Rect::new(334, 200, 334, 200), rect);
    }
}
