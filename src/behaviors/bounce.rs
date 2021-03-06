use crate::{physics::Physics, Contextable, Ticker};

pub struct Bounce {}

impl<C: Contextable> Ticker<C> for Bounce {
    fn tick(&self, input: Physics<C>, dimension: &(u32, u32)) -> Physics<C> {
        Physics::new(
            input.x + input.vx,
            input.y + input.vy,
            input.r,
            if input.x + input.vx > dimension.0.into() || input.x + input.vx < 0.0 {
                -input.vx
            } else {
                input.vx
            },
            if input.y + input.vy > dimension.1.into() || input.y + input.vy < 0.0 {
                -input.vy
            } else {
                input.vy
            },
        )
    }
}
