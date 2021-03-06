use crate::{physics::Physics, Contextable, Ticker};

pub struct Drift {}

impl<C: Contextable> Ticker<C> for Drift {
    fn tick(&self, input: Physics<C>, _dimension: &(u32, u32)) -> Physics<C> {
        Physics::new(
            input.x + input.vx,
            input.y + input.vy,
            input.r,
            input.vx,
            input.vy,
        )
    }
}
