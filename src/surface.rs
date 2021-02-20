use crate::Scene;

const SCALE: f64 = 400.0;

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
    }
}
