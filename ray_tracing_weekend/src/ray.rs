use crate::vec3::*;

#[derive(Copy, Debug, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
