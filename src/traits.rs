use sdl2::render::Renderer;

pub trait Renderable {
    // TODO: make Renderable::render() return Result iso ()
    fn render(&self, renderer: &mut Renderer);
}

pub trait Updatable {
    fn update(&mut self, delta_time: u32);
}
