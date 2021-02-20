mod behavior;
mod physics;
pub mod surface;

use crate::behavior::behavior::Behavior;
pub use behavior::scene::Scene;
use physics::Physics;

trait Ticker {
    fn tick(&self, input: Physics) -> Physics;
}

pub struct Blob {
    physics: Physics,
    behavior: Behavior,
}

impl Blob {
    pub fn new(physics: Physics, behavior: Behavior) -> Self {
        Self { physics, behavior }
    }

    pub fn x(&self) -> f64 {
        self.physics.x
    }

    pub fn y(&self) -> f64 {
        self.physics.y
    }

    pub fn r(&self) -> f64 {
        self.physics.r
    }

    pub fn tick(self) -> Blob {
        let Self { physics, behavior } = self;
        Blob::new(behavior.tick(physics), behavior)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
