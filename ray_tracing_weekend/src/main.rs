use std::ops;

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

fn dot(lhs : Vec3, rhs: Vec3) -> f64{
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

fn cross(lhs : Vec3, rhs: Vec3) -> f64{
    0.0
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("255");

    for y in 0..image_height{
        eprintln!("scanlines remaining: {0}", y);
        for x in 0..image_width{
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
    eprintln!("Done.");
}
