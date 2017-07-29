use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::rect::Point;

use screen::Screen;
use midi::AbsMidiEvent;
use config::Config;

pub struct KeyboardLayoutScreen {
    config: Config,
    quit: bool
}

impl KeyboardLayoutScreen {
    pub fn new(config: Config) -> KeyboardLayoutScreen {
        KeyboardLayoutScreen {
            config: config,
            quit: false
        }
    }
}

// TODO: implement Screen trait for KeyboardLayoutScreen
impl Screen<Config> for KeyboardLayoutScreen {
    fn handle_sdl_events(&mut self, events: &[Event]) {
        for event in events {
            match *event {
                Event::Quit { .. } => {
                    self.quit = true
                },

                _ => {}
            }
        }
    }

    fn handle_midi_events(&mut self, events: &[AbsMidiEvent]) {
    }

    fn update(&mut self, delta_time: u32) -> Option<Config> {
        if self.quit {
            Some(self.config.clone())
        } else {
            None
        }
    }

    fn render(&self, renderer: &mut Renderer) {
        let window_width = renderer.viewport().width() as i32;
        let window_height = renderer.viewport().height() as i32;

        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.draw_line(Point::from((0, 0)), Point::from((window_width, window_height)));
        renderer.draw_line(Point::from((window_width, 0)), Point::from((0, window_height)));
    }
}
