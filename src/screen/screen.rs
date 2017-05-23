use sdl2::event::Event;
use sdl2::render::{Renderer};
use midi::AbsMidiEvent;
use screen::StateId;

// TODO: Incorporate configuration passing into the state graph
//
// Essentially the program configuration should be read on the start
// of the application, be passed between the states and then persistet
// back when the application is finished. During the passing the
// states may mutate the configuration as they see fit.
pub trait Screen {
    fn handle_sdl_events(&mut self, events: &[Event]);
    fn handle_midi_events(&mut self, events: &[AbsMidiEvent]);
    fn update(&mut self, delta_time: u32) -> StateId;
    fn render(&self, renderer: &mut Renderer);
}
