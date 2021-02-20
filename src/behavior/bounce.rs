use crate::{physics::Physics, Ticker};

pub struct Bounce {}

impl Ticker for Bounce {
    fn tick(&self, input: Physics) -> Physics {
        Physics::new(
            input.x + input.vx,
            input.y + input.vy,
            input.r,
            if input.x + input.vx > 1024.0 || input.x + input.vx < 0.0 {
                -input.vx
            } else {
                input.vx
            },
            if input.y + input.vy > 768.0 || input.y + input.vy < 0.0 {
                -input.vy
            } else {
                input.vy
            },
        )
    }
}
