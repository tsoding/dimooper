use sdl2::render::Renderer;

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);
}
