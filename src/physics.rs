pub struct Physics {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub vx: f64,
    pub vy: f64,
}

impl Physics {
    pub fn new(x: f64, y: f64, r: f64, vx: f64, vy: f64) -> Self {
        Self { x, y, r, vx, vy }
    }
}
