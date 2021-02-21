use crate::{behaviors::behavior::Behavior, physics::Physics, Ticker};

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

    pub fn tick(self, dimension: &(u32, u32)) -> Blob {
        let Self { physics, behavior } = self;
        Blob::new(behavior.tick(physics, dimension), behavior)
    }
}
