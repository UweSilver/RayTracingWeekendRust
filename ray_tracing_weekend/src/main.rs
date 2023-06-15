use std::{ops, intrinsics::sqrtf64};

struct Vec3{
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3{
        let output: Vec3 = Vec3 { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z) };
        output
    }
}

impl ops::Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, _rhs: f64) -> Self::Output{
        Self::Output{x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs}
    }
}

impl ops::Mul<Vec3> for f64{
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Self::Output {
        Self::Output{x: self * _rhs.x, y: self * _rhs.y, z: self * _rhs.z}
    }
}

fn dot(lhs : Vec3, rhs: Vec3) -> f64{
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

fn cross(lhs : Vec3, rhs: Vec3) -> Vec3{
    Vec3{x: lhs.y * rhs.z - lhs.z *rhs.y, y: lhs.z * rhs.x - lhs.x * rhs.z, z: lhs.x * rhs.y - lhs.y * rhs.x}
}

fn length(lhs: Vec3) -> f64{
    f64::sqrt(lhs.x * lhs.x + lhs.y * lhs.y + lhs.z * lhs.z)
}

impl std::fmt::Display for Vec3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

type colour = Vec3;
type point3 = Vec3;

fn write_colour(c: colour){
    println!("{} {} {}", (255.999 * c.x) as i32, (255.999 * c.y) as i32, (255.999 * c.z) as i32);
}

struct Ray{
    origin: point3,
    dir: Vec3
}

trait Point {
    fn at(self, t: f64) -> point3;
}

impl Point for Ray{
    fn at(self, t: f64) -> point3{
        self.origin + t * self.dir
    }
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("255");

    for j in 0..image_height{
        eprintln!("scanlines remaining: {0}", j);
        for i in 0..image_width{
            
            let pixel_colour = colour{x: i as f64 / (image_width - 1) as f64, y: j as f64 / (image_height - 1) as f64, z: 0.25};
            write_colour(pixel_colour);
        }
    }
    eprintln!("Done.");
}
