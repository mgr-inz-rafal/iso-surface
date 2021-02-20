use crate::{physics::Physics, Ticker};

use super::{bounce::Bounce, drift::Drift};

pub enum Behavior {
    Drift(Drift),
    Bounce(Bounce),
}

impl Behavior {
    pub fn new_drift() -> Behavior {
        Behavior::Drift(Drift {})
    }

    pub fn new_bounce() -> Behavior {
        Behavior::Bounce(Bounce {})
    }
}

impl Ticker for Behavior {
    fn tick(&self, input: Physics, dimension: &(u32, u32)) -> Physics {
        match self {
            Behavior::Drift(implementation) => implementation.tick(input, dimension),
            Behavior::Bounce(implementation) => implementation.tick(input, dimension),
        }
    }
}
