use crate::{behaviors::behavior::Behavior, physics::Physics, Contextable, Ticker};

pub struct Blob<C: Contextable> {
    physics: Physics<C>,
    behavior: Behavior<C>,
}

impl<C: Contextable> Blob<C> {
    pub fn new(physics: Physics<C>, behavior: Behavior<C>) -> Self {
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

    pub fn tick(self, dimension: &(u32, u32)) -> Blob<C> {
        let Self { physics, behavior } = self;
        Blob::new(behavior.tick(physics, dimension), behavior)
    }
}
