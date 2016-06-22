use sdl2::render::Renderer;
use sdl2::rect::Point;

pub trait GraphicsPrimitives {
    fn fill_circle(&mut self, cx: i32, cy: i32, r: i32);
    fn draw_circle(&mut self, cx: i32, cy: i32, r: i32);
}

impl<'a> GraphicsPrimitives for Renderer<'a> {
    fn fill_circle(&mut self, cx: i32, cy: i32, r: i32) {
        let mut x = r;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            self.draw_line(Point::new(cx + x, cy - y), Point::new(cx + x, cy + y)).unwrap();
            self.draw_line(Point::new(cx + y, cy - x), Point::new(cx + y, cy + x)).unwrap();
            self.draw_line(Point::new(cx - y, cy - x), Point::new(cx - y, cy + x)).unwrap();
            self.draw_line(Point::new(cx - x, cy - y), Point::new(cx - x, cy + y)).unwrap();

            y += 1;
            err += 1 + 2*y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }

    fn draw_circle(&mut self, cx: i32, cy: i32, r: i32) {
        let mut x = r;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            self.draw_point(Point::new(cx + x, cy - y)).unwrap();
            self.draw_point(Point::new(cx + x, cy + y)).unwrap();
            self.draw_point(Point::new(cx + y, cy - x)).unwrap();
            self.draw_point(Point::new(cx + y, cy + x)).unwrap();
            self.draw_point(Point::new(cx - y, cy - x)).unwrap();
            self.draw_point(Point::new(cx - y, cy + x)).unwrap();
            self.draw_point(Point::new(cx - x, cy - y)).unwrap();
            self.draw_point(Point::new(cx - x, cy + y)).unwrap();

            y += 1;
            err += 1 + 2*y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }
}
