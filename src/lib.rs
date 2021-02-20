use itertools::Itertools;

const SCALE: f64 = 400.0;

pub struct Physics {
    x: f64,
    y: f64,
    r: f64,
    vx: f64,
    vy: f64,
}

impl Physics {
    pub fn new(x: f64, y: f64, r: f64, vx: f64, vy: f64) -> Self {
        Self { x, y, r, vx, vy }
    }
}

trait Ticker {
    fn tick(&self, input: Physics) -> Physics;
}

pub struct Drift {}

impl Ticker for Drift {
    fn tick(&self, input: Physics) -> Physics {
        Physics::new(
            input.x + input.vx,
            input.y + input.vy,
            input.r,
            input.vx,
            input.vy,
        )
    }
}

pub enum Behavior {
    Drift(Drift),
}

impl Behavior {
    pub fn new_drift() -> Behavior {
        let x = Drift {};
        Behavior::Drift(x)
    }
}

impl Ticker for Behavior {
    fn tick(&self, input: Physics) -> Physics {
        match self {
            Behavior::Drift(drift_impl) => drift_impl.tick(input),
        }
    }
}

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

    pub fn tick(self) -> Blob {
        let Self { physics, behavior } = self;
        Blob::new(behavior.tick(physics), behavior)
    }
}

pub struct Scene {
    blobs: Vec<Blob>,
}

impl Scene {
    pub fn new() -> Self {
        Self { blobs: vec![] }
    }

    pub fn add_blob<T>(&mut self, x: T, y: T, r: T, vx: T, vy: T)
    where
        T: Into<f64>,
    {
        self.blobs.push(Blob::new(
            Physics::new(x.into(), y.into(), r.into(), vx.into(), vy.into()),
            Behavior::new_drift(),
        ));
    }

    pub fn blobs(&self) -> impl Iterator<Item = &Blob> {
        self.blobs.iter()
    }

    pub fn tick(&mut self) {
        self.blobs = self.blobs.drain(..).map(|blob| blob.tick()).collect_vec();
    }
}

pub struct Surface {
    dimension: (u32, u32),
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            dimension: (width, height),
        }
    }

    pub fn width(&self) -> u32 {
        self.dimension.0
    }

    pub fn height(&self) -> u32 {
        self.dimension.1
    }

    pub fn max_distance(&self) -> f64 {
        let p1 = nalgebra::Point2::new(0 as f64, 0 as f64);
        let p2 = nalgebra::Point2::new(self.width() as f64, self.height() as f64);
        nalgebra::distance(&p1, &p2)
    }

    pub fn pixel_color(scene: &Scene, x: i32, y: i32) -> (u8, u8, u8) {
        // TODO: Calculate HTML-color here, so there's no need to call rgb() on JS

        let p1 = nalgebra::Point2::new(x as f64, y as f64);
        let mut sum = 0.0;
        for blob in scene.blobs() {
            let p2 = nalgebra::Point2::new(blob.x(), blob.y());
            sum += SCALE * blob.r() / nalgebra::distance(&p1, &p2);
        }
        (0, sum as u8, 0)

        // let mut sum = 0.0;
        // for blob in scene.blobs() {
        //     let distance = ((blob.x() - x as f64) * (blob.x() - x as f64)
        //         + (blob.y() - y as f64) * (blob.y() - y as f64))
        //         .sqrt();
        //     sum += SCALE * blob.r() / distance;
        // }

        // (0, sum as u8, 0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
