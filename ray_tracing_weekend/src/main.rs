mod vec3;
use vec3::*;

mod math_util;
use math_util::*;

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

#[derive(Clone, Copy, Debug)]
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
            - camera.vertical / 2.0
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        camera
    }
}

fn get_ray(camera: Camera, u: f64, v: f64) -> Ray {
    Ray {
        origin: camera.origin,
        dir: camera.lower_left_corner + u * camera.horizontal + v * camera.vertical - camera.origin,
    }
}

fn ray_colour(ray: Ray, hittable: Box<&dyn Hittable>, depth: i32) -> Colour {
    if (depth <= 0) {
        return Colour {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    match hittable.hit(ray, 0.001, infinite()) {
        Some(record) => {
            let target = record.p + random_in_hemisphere(record.normal);
            return 0.5
                * (ray_colour(
                    Ray {
                        origin: record.p,
                        dir: target - record.p,
                    },
                    hittable,
                    depth - 1,
                ));
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

fn write_colour(pixel_colour: Colour, samples_per_pixel: i32) {
    let mut r = pixel_colour.x;
    let mut g = pixel_colour.y;
    let mut b = pixel_colour.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    println!(
        "{} {} {}",
        (255.999 * r) as i32,
        (255.999 * g) as i32,
        (255.999 * b) as i32
    );
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 768;
    //let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let depth = 50;

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("255");

    let camera = Camera::default();

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

    let bar = indicatif::ProgressBar::new(image_height as u64);

    for j in (0..image_height).rev() {
        //eprintln!("scanlines remaining: {0}", j);
        for i in 0..image_width {
            let mut pixel_colour: Colour = Colour::default();
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + f64::random()) / (image_width - 1) as f64;
                let v = (j as f64 + f64::random()) / (image_height - 1) as f64;

                let r = get_ray(camera, u, v);
                pixel_colour += ray_colour(r, Box::new(&world), depth);
            }
            write_colour(pixel_colour, samples_per_pixel);
        }

        bar.inc(1);
    }
    eprintln!("Done.");
}
