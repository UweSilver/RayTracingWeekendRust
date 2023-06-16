use crate::vec3::*;

#[derive(Copy, Debug, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

pub trait Point {
    fn at(self, t: f64) -> Point3;
}

impl Point for Ray {
    fn at(self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
