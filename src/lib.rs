mod behavior;
mod blob;
mod physics;
pub mod surface;

pub use behavior::scene::Scene;
use physics::Physics;

trait Ticker {
    fn tick(&self, input: Physics, dimension: &(u32, u32)) -> Physics;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
