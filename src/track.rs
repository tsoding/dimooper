use pm::types::MidiEvent;
use sdl2::render::Renderer;

pub struct Track {

}

impl Track {
    fn update(&mut self, delta_time: u32) {
        unimplemented!()
    }

    fn on_midi_event(&mut self, event: &MidiEvent) {
        unimplemented!()
    }

    fn render(&mut self, renderer: &mut Renderer) {
        unimplemented!()
    }
}
