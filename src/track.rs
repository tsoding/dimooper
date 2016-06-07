use pm::types::MidiEvent;
use sdl2::render::Renderer;
use ::updatable::Updatable;

pub struct Track {

}

impl Updatable for Track {
    fn update(&mut self, delta_time: u32) {
        unimplemented!()
    }
}

impl Track {
    fn on_midi_event(&mut self, event: &MidiEvent) {
        unimplemented!()
    }

    fn render(&mut self, renderer: &mut Renderer) {
        unimplemented!()
    }
}
