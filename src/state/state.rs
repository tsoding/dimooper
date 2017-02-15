use sdl2::event::Event;
use sdl2::render::{Renderer};
use midi::AbsMidiEvent;
use state::StateId;

pub trait State {
    fn handle_sdl_events(&mut self, events: &[Event]);
    fn handle_midi_events(&mut self, events: &[AbsMidiEvent]);
    fn update(&mut self, delta_time: u32) -> StateId;
    fn render(&self, renderer: &mut Renderer);
}
