pub trait Random {
    type Output;
    fn random() -> Self::Output;
    fn random_range(min: f64, max: f64) -> Self::Output;
}

impl Random for f64 {
    type Output = f64;
    fn random() -> Self::Output {
        rand::random()
    }
    fn random_range(min: f64, max: f64) -> Self::Output {
        min + (max - min) * Self::random()
    }
}

pub fn infinite() -> f64 {
    f64::MAX
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
