
pub trait MidRound {
    fn mid(&self) -> Self;
}

impl MidRound for f32 {
    fn mid(&self) -> Self {
        match self {
            v if v > &0.0 => v.floor(),
            v if v < &0.0 => v.ceil(),
            v => *v,
        }
    }
}

impl MidRound for f64 {
    fn mid(&self) -> Self {
        match self {
            v if v > &0.0 => v.floor(),
            v if v < &0.0 => v.ceil(),
            v => *v,
        }
    }
}

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2-x1).powf(2.0) + (y2-y1).powf(2.0)).sqrt()
}