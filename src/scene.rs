use itertools::Itertools;

use crate::{
    behaviors::behavior::Behavior, blob::Blob, physics::Physics, surface::Surface, Contextable,
    Ticker,
};

pub struct Scene<C: Contextable> {
    dimension: (u32, u32),
    blobs: Vec<Blob<C>>,
}

impl<C: Contextable> Scene<C> {
    pub fn new(surface: &Surface) -> Self {
        Self {
            dimension: (surface.width(), surface.height()),
            blobs: vec![],
        }
    }

    pub fn add_drifter<T>(&mut self, x: T, y: T, r: T, vx: T, vy: T)
    where
        T: Into<f64>,
    {
        self.blobs.push(Blob::new(
            Physics::new(x.into(), y.into(), r.into(), vx.into(), vy.into()),
            Behavior::new_drift(),
        ));
    }

    pub fn add_bouncer<T>(&mut self, x: T, y: T, r: T, vx: T, vy: T)
    where
        T: Into<f64>,
    {
        self.blobs.push(Blob::new(
            Physics::new(x.into(), y.into(), r.into(), vx.into(), vy.into()),
            Behavior::new_bounce(),
        ));
    }

    pub fn add_custom<T>(
        &mut self,
        x: T,
        y: T,
        r: T,
        vx: T,
        vy: T,
        context: C,
        ticker: Box<dyn Ticker<C>>,
    ) where
        T: Into<f64>,
    {
        self.blobs.push(Blob::new(
            Physics::with_context(x.into(), y.into(), r.into(), vx.into(), vy.into(), context),
            Behavior::new_custom(ticker),
        ));
    }

    pub fn blobs(&self) -> impl Iterator<Item = &Blob<C>> {
        self.blobs.iter()
    }

    pub fn tick(&mut self) {
        let dimension = &self.dimension;
        self.blobs = self
            .blobs
            .drain(..)
            .map(|blob| blob.tick(&dimension))
            .collect_vec();
    }
}
