use itertools::Itertools;

use crate::{blob::Blob, physics::Physics};

use super::behavior::Behavior;

pub struct Scene {
    blobs: Vec<Blob>,
}

impl Scene {
    pub fn new() -> Self {
        Self { blobs: vec![] }
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

    pub fn blobs(&self) -> impl Iterator<Item = &Blob> {
        self.blobs.iter()
    }

    pub fn tick(&mut self) {
        self.blobs = self.blobs.drain(..).map(|blob| blob.tick()).collect_vec();
    }
}
