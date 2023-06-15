use std::{ops};

#[derive(Copy, Debug, Clone)]
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

impl ops::Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3{
        let output: Vec3 = Vec3 { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z) };
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

impl ops::Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Self::Output{
        Self::Output{x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs}
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

fn unit_vector(lhs: Vec3) -> Vec3{
    lhs / length(lhs)
}

impl std::fmt::Display for Vec3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

type Colour = Vec3;
type Point3 = Vec3;

fn write_colour(c: Colour){
    println!("{} {} {}", (255.999 * c.x) as i32, (255.999 * c.y) as i32, (255.999 * c.z) as i32);
}

#[derive(Copy, Debug, Clone)]
struct Ray{
    origin: Point3,
    dir: Vec3
}

trait Point {
    fn at(self, t: f64) -> Point3;
}

impl Point for Ray{
    fn at(self, t: f64) -> Point3{
        self.origin + t * self.dir
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> bool{
    let oc = ray.origin - center;
    let a = dot(ray.dir, ray.dir);
    let b = 2.0 * dot(oc, ray.dir);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_colour(ray: Ray) -> Colour{
    if hit_sphere(Point3{x: 0.0, y: 0.0, z: -1.0}, 0.5, ray){
        return Colour{x: 1.0, y: 0.0, z: 0.0}
    }
    let unit_direction: Vec3 = unit_vector(ray.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Colour{x: 1.0, y: 1.0, z: 1.0} + t * Colour{x: 0.5, y: 0.7, z: 1.0}
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("255");

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0- Vec3{x: 0.0, y: 0.0, z: focal_length};

    for j in (0..image_height).rev(){
        eprintln!("scanlines remaining: {0}", j);
        for i in 0..image_width{
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray{origin: origin, dir: lower_left_corner + u * horizontal + v * vertical - origin};
            let pixel_colour = ray_colour(r);
            write_colour(pixel_colour);
        }
    }
    eprintln!("Done.");
}
