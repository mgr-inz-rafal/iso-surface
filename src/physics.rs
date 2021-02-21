pub struct Physics {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub vx: f64,
    pub vy: f64,
}

impl Physics {
    pub fn new<T: Into<f64>>(x: T, y: T, r: T, vx: T, vy: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            r: r.into(),
            vx: vx.into(),
            vy: vy.into(),
        }
    }
}
