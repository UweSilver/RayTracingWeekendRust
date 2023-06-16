use rand::prelude::*;

pub fn random() -> f64 {
    rand::random()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}

pub fn infinite() -> f64 {
    f64::MAX
}
