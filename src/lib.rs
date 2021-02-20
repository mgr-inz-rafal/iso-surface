const SCALE: f64 = 400.0;

pub struct Blob {
    x: f64,
    y: f64,
    r: f64,
    vx: f64,
    vy: f64,
}

impl Blob {
    pub fn new(x: f64, y: f64, r: f64, vx: f64, vy: f64) -> Self {
        Self { x, y, r, vx, vy }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn tick(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.x > 1024.0 || self.x < 0.0 {
            self.vx = -self.vx;
            //self.x += self.vx;
        }

        if self.y > 768.0 || self.y < 0.0 {
            self.vy = -self.vy;
            //self.y += self.vy;
        }
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
            x.into(),
            y.into(),
            r.into(),
            vx.into(),
            vy.into(),
        ));
    }

    pub fn blobs(&self) -> impl Iterator<Item = &Blob> {
        self.blobs.iter()
    }

    pub fn tick(&mut self) {
        self.blobs.iter_mut().for_each(|blob| blob.tick());
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
