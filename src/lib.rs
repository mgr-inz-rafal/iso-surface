mod behaviors;
mod blob;
pub mod physics;
pub mod scene;
pub mod surface;

pub use physics::Physics;

pub trait Ticker<C: Contextable> {
    fn tick(&self, input: Physics<C>, dimension: &(u32, u32)) -> Physics<C>;
}

pub trait Contextable {
    fn frame(&self) -> usize;
    fn set_frame(&mut self, frame: usize);
    fn should_advance(&mut self) -> bool;
}
