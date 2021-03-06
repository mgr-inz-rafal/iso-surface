use crate::{physics::Physics, Contextable, Ticker};

use super::{bounce::Bounce, drift::Drift};

pub enum Behavior<C: Contextable> {
    Drift(Drift),
    Bounce(Bounce),
    Custom(Box<dyn Ticker<C>>),
}

impl<C: Contextable> Behavior<C> {
    pub fn new_drift() -> Behavior<C> {
        Behavior::Drift(Drift {})
    }

    pub fn new_bounce() -> Behavior<C> {
        Behavior::Bounce(Bounce {})
    }

    pub fn new_custom(ticker: Box<dyn Ticker<C>>) -> Behavior<C> {
        Behavior::Custom(ticker)
    }
}

impl<C: Contextable> Ticker<C> for Behavior<C> {
    fn tick(&self, input: Physics<C>, dimension: &(u32, u32)) -> Physics<C> {
        match self {
            Behavior::Drift(implementation) => implementation.tick(input, dimension),
            Behavior::Bounce(implementation) => implementation.tick(input, dimension),
            Behavior::Custom(implementation) => implementation.tick(input, dimension),
        }
    }
}
