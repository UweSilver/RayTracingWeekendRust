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

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = f64::powf((1.0 - ref_idx) / (1.0 + ref_idx), 2.0);
    r0 + (1.0 - r0) * f64::powf((1.0 - cosine), 5.0)
}
