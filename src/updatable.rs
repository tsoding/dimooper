pub trait Updatable {
    fn update(&mut self, delta_time: u32);
}
