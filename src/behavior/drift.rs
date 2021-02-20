use crate::{physics::Physics, Ticker};

pub struct Drift {}

impl Ticker for Drift {
    fn tick(&self, input: Physics, _dimension: &(u32, u32)) -> Physics {
        Physics::new(
            input.x + input.vx,
            input.y + input.vy,
            input.r,
            input.vx,
            input.vy,
        )
    }
}
