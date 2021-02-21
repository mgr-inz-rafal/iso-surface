mod behavior;
mod blob;
mod physics;
pub mod scene;
pub mod surface;

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
