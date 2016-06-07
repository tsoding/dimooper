use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Renderer;

pub struct Arkanoid {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    width: u32,
    height: u32,
    color: Color,
}

impl Default for Arkanoid {
    fn default() -> Arkanoid {
        Arkanoid {
            x: f32::default(),
            y: f32::default(),
            vx: 1.0,
            vy: 1.0,
            width: 100,
            height: 100,
            color: Color::RGB(255, 0, 0),
        }
    }
}

impl Arkanoid {
    pub fn update(&mut self, window_width: u32, window_height: u32) {
        let velocity = 0.1;

        if self.x < 0.0 || self.x > (window_width - self.width) as f32 {
            self.vx = -self.vx;
        }

        if self.y < 0.0 || self.y > (window_height - self.height) as f32 {
            self.vy = -self.vy;
        }

        self.x += self.vx * velocity;
        self.y += self.vy * velocity;
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        renderer.set_draw_color(self.color);
        renderer.fill_rect(Rect::new(self.x as i32, self.y as i32, self.width, self.height)).unwrap();
        renderer.present();
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
