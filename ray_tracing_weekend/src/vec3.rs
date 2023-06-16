use std::{f64::consts::PI, ops};

use crate::math_util::{self, Random};

#[derive(Copy, Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        let output: Vec3 = Vec3 {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
        };
        output
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        let output: Vec3 = Vec3 {
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
            z: (self.z - rhs.z),
        };
        output
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: lhs.z * rhs.x - lhs.x * rhs.z,
        z: lhs.x * rhs.y - lhs.y * rhs.x,
    }
}

pub fn length_squared(lhs: Vec3) -> f64 {
    lhs.x * lhs.x + lhs.y * lhs.y + lhs.z * lhs.z
}

pub fn length(lhs: Vec3) -> f64 {
    f64::sqrt(length_squared(lhs))
}

pub fn unit_vector(lhs: Vec3) -> Vec3 {
    lhs / length(lhs)
}

impl math_util::Random for Vec3 {
    type Output = Vec3;
    fn random() -> Vec3 {
        Vec3 {
            x: f64::random(),
            y: f64::random(),
            z: f64::random(),
        }
    }
    fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: f64::random_range(min, max),
            y: f64::random_range(min, max),
            z: f64::random_range(min, max),
        }
    }
}

pub fn random_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if length_squared(p) > 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub fn random_unit_vec3() -> Vec3 {
    let a = f64::random_range(0.0, 2.0 * PI);
    let z = f64::random_range(-1.0, 1.0);
    let r = f64::sqrt(1.0 - z * z);
    Vec3 {
        x: r * f64::cos(a),
        y: r * f64::sin(a),
        z: z,
    }
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_vec3_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

pub type Colour = Vec3;
pub type Point3 = Vec3;
