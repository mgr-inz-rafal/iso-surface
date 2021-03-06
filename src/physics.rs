use crate::Contextable;

pub struct Physics<C: Contextable> {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub vx: f64,
    pub vy: f64,
    pub context: Option<C>,
}

impl<C: Contextable> Physics<C> {
    pub fn new<T>(x: T, y: T, r: T, vx: T, vy: T) -> Self
    where
        T: Into<f64>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            r: r.into(),
            vx: vx.into(),
            vy: vy.into(),
            context: None,
        }
    }

    pub fn with_context<T>(x: T, y: T, r: T, vx: T, vy: T, context: C) -> Self
    where
        T: Into<f64>,
        C: Contextable,
    {
        Self {
            x: x.into(),
            y: y.into(),
            r: r.into(),
            vx: vx.into(),
            vy: vy.into(),
            context: Some(context),
        }
    }
}
