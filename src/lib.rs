mod behavior;
mod blob;
pub mod physics;
pub mod scene;
pub mod surface;

pub use physics::Physics;

pub trait Ticker {
    fn tick(&self, input: Physics, dimension: &(u32, u32)) -> Physics;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
