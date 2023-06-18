use std::rc::Rc;

mod vec3;
use vec3::*;

mod math_util;
use math_util::*;

mod ray;
use ray::*;

trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Colour, Ray)>;
}

struct Lambertian {
    albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Colour, Ray)> {
        let scatter_direction = rec.normal + random_unit_vec3();
        let scattered = Ray {
            origin: rec.p,
            dir: scatter_direction,
        };
        Some((self.albedo, scattered))
    }
}

struct Metal {
    albedo: Colour,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Colour, Ray)> {
        let reflected = reflect(r_in.dir.get_normalized(), rec.normal);
        let scattered = Ray {
            origin: rec.p,
            dir: reflected,
        };
        let attenuation = self.albedo;

        if dot(scattered.dir, rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    material: Rc<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: f64::default(),
            material: Rc::new(Lambertian {
                albedo: Vec3::default(),
            }),
        }
    }
}

trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = dot(oc, ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
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
                    material: Rc::clone(&self.material),
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
                    material: Rc::clone(&self.material),
                });
            }
        }
        return None;
    }
}

struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
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
    if depth <= 0 {
        return Colour {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    match hittable.hit(ray, 0.001, infinite()) {
        Some(record) => match record.material.scatter(ray, record.clone()) {
            Some((attenuation, scattered)) => {
                return attenuation * ray_colour(scattered, hittable, depth - 1);
            }
            None => {
                return Colour::default();
            }
        },
        None => {}
    }

    let unit_direction: Vec3 = ray.dir.get_normalized();
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
            Rc::new(Sphere {
                center: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
                material: Rc::new(Lambertian {
                    albedo: Colour {
                        x: 0.7,
                        y: 0.3,
                        z: 0.3,
                    },
                }),
            }),
            Rc::new(Sphere {
                center: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
                material: Rc::new(Metal {
                    albedo: Colour {
                        x: 0.8,
                        y: 0.6,
                        z: 0.2,
                    },
                }),
            }),
            Rc::new(Sphere {
                center: Vec3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
                material: Rc::new(Metal {
                    albedo: Colour {
                        x: 0.8,
                        y: 0.8,
                        z: 0.8,
                    },
                }),
            }),
            Rc::new(Sphere {
                center: Vec3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                radius: 100.0,
                material: Rc::new(Lambertian {
                    albedo: Colour {
                        x: 0.1,
                        y: 0.7,
                        z: 0.1,
                    },
                }),
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
