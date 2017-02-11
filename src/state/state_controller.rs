use sdl2::event::Event;
use sdl2::render::{Renderer};
use midi::AbsMidiEvent;
use state::State;

pub trait StateController {
    fn handle_sdl_events(&mut self, events: &[Event]);
    fn handle_midi_events(&mut self, events: &[AbsMidiEvent]);
    fn update(&mut self, delta_time: u32) -> State;
    fn render(&self, renderer: &mut Renderer);
}
