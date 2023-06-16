use rand::prelude::*;
use std::{default, ops};

mod vec3;
use vec3::*;

mod math_util;
use math_util::*;

fn write_colour(c: Colour) {
    println!(
        "{} {} {}",
        (255.999 * c.x) as i32,
        (255.999 * c.y) as i32,
        (255.999 * c.z) as i32
    );
}

#[derive(Copy, Debug, Clone)]
struct Ray {
    origin: Point3,
    dir: Vec3,
}

trait Point {
    fn at(self, t: f64) -> Point3;
}

impl Point for Ray {
    fn at(self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin - center;
    let a = length_squared(ray.dir);
    let half_b = dot(oc, ray.dir);
    let c = length_squared(oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

#[derive(Clone, Copy, Debug)]
struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: f64::default(),
        }
    }
}

trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = length_squared(ray.dir);
        let half_b = dot(oc, ray.dir);
        let c = length_squared(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: normal,
                });
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: t,
                    p: p,
                    normal: normal,
                });
            }
        }
        return None;
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record = HitRecord::default();
        let mut hit_anithing = false;
        let mut closest_so_far = t_max;

        self.objects
            .iter()
            .for_each(|object| match object.hit(ray, t_min, closest_so_far) {
                Some(object_hit_record) => {
                    record = object_hit_record.clone();
                    hit_anithing = true;
                    closest_so_far = object_hit_record.t;
                }
                None => {}
            });

        if hit_anithing {
            Some(record)
        } else {
            None
        }
    }
}

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let mut camera = Camera {
            origin: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            horizontal: Vec3 {
                x: viewport_width,
                y: 0.0,
                z: 0.0,
            },
            vertical: Vec3 {
                x: 0.0,
                y: viewport_height,
                z: 0.0,
            },
            lower_left_corner: Vec3::default(),
        };
        camera.lower_left_corner = camera.origin
            - camera.horizontal / 2.0
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        camera
    }
}

fn ray_colour(ray: Ray, hittable: Box<&dyn Hittable>) -> Colour {
    match hittable.hit(ray, 0.0, infinite()) {
        Some(record) => {
            return 0.5
                * (record.normal
                    + Colour {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    });
        }
        None => {}
    }

    let unit_direction: Vec3 = unit_vector(ray.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t)
        * Colour {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * Colour {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
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

    let origin = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    let world = HittableList {
        objects: vec![
            Box::new(Sphere {
                center: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                radius: 100.0,
            }),
        ],
    };

    for j in (0..image_height).rev() {
        eprintln!("scanlines remaining: {0}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray {
                origin: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_colour = ray_colour(r, Box::new(&world));
            write_colour(pixel_colour);
        }
    }
    eprintln!("Done.");
}
