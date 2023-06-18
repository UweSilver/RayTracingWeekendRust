use std::{f64::consts::PI, rc::Rc};

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
    fuzz: f64,//[0, 1]
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Colour, Ray)> {
        let reflected = reflect(r_in.dir.get_normalized(), rec.normal);
        let scattered = Ray {
            origin: rec.p,
            dir: reflected + self.fuzz * random_vec3_in_unit_sphere(),
        };
        let attenuation = self.albedo;

        if dot(scattered.dir, rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

struct Dielectric {
    ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Colour, Ray)> {
        let attenuation = Colour {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.dir.get_normalized();
        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, rec.normal);
            Some((
                attenuation,
                Ray {
                    origin: rec.p,
                    dir: reflected,
                },
            ))
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            if f64::random() < reflect_prob {
                let reflected = reflect(unit_direction, rec.normal);
                Some((
                    attenuation,
                    Ray {
                        origin: rec.p,
                        dir: reflected,
                    },
                ))
            } else {
                let refracted = refract(unit_direction, rec.normal, etai_over_etat);
                Some((
                    attenuation,
                    Ray {
                        origin: rec.p,
                        dir: refracted,
                    },
                ))
            }
        }
    }
}

#[derive(Clone)]
struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: Rc<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: f64::default(),
            front_face: false,
            material: Rc::new(Lambertian {
                albedo: Vec3::default(),
            }),
        }
    }
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
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
                let mut record = HitRecord::default();
                record.t = temp;
                record.p = ray.at(record.t);
                record.material = Rc::clone(&self.material);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_face_normal(ray, outward_normal);
                return Some(record);
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let mut record = HitRecord::default();
                record.t = temp;
                record.p = ray.at(record.t);
                record.material = Rc::clone(&self.material);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_face_normal(ray, outward_normal);
                return Some(record);
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
    lens_radius: f64,
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
            lens_radius: 1.0,
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

fn create_camera(
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    vfov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
) -> Camera {
    let theta = f64::to_radians(vfov);
    let h = f64::tan(theta / 2.0);
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (lookfrom - lookat).get_normalized();
    let u = cross(vup, w).get_normalized();
    let v = cross(w, u);

    let origin = lookfrom;
    let horizontal = viewport_width * u * focus_dist;
    let vertical = viewport_height * v * focus_dist;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

    let lens_radius = aperture / 2.0;

    Camera {
        origin: origin,
        lower_left_corner: lower_left_corner,
        horizontal: horizontal,
        vertical: vertical,
        lens_radius: lens_radius,
    }
}

fn get_ray(camera: Camera, s: f64, t: f64) -> Ray {
    let rd = camera.lens_radius * random_in_unit_disk();
    let offset = Vec3{x: s * rd.x, y: t * rd.y, z: 0.0};
    Ray {
        origin: camera.origin + offset,
        dir: camera.lower_left_corner + s * camera.horizontal + t * camera.vertical - camera.origin - offset,
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
    let image_width = 384;
    //let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let depth = 50;

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("255");

    let lookfrom = Point3{x: -2.0, y: 2.0, z: 1.0};
    let lookat =  Point3{x: 0.0, y:0.0, z:-1.0};
    let vup = Vec3{x: 0.0, y: 1.0, z: 0.0};
    let camera = create_camera(lookfrom, lookat, vup, 20.0, image_width as f64 / image_height as f64, 2.0, (lookfrom - lookat).length());

    let world = HittableList {
        objects: vec![
            Rc::new(Sphere {
                center: Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
                material: Rc::new(Lambertian {
                    albedo: Colour {
                        x: 0.1,
                        y: 0.2,
                        z: 0.5,
                    },
                }),
            }),
            Rc::new(Sphere {
                center: Point3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                radius: 100.0,
                material: Rc::new(Lambertian {
                    albedo: Colour {
                        x: 0.8,
                        y: 0.8,
                        z: 0.0,
                    },
                }),
            }),
            Rc::new(Sphere {
                center: Point3 {
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
                    fuzz: 0.3
                }),
            }),
            Rc::new(Sphere {
                center: Point3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
                material: Rc::new(Dielectric{ref_idx: 1.5}),
            }),
            Rc::new(Sphere {
                center: Point3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: -0.45,
                material: Rc::new(Dielectric{ref_idx: 1.5}),
            }),
        ],
    };

    let bar = indicatif::ProgressBar::new(image_height as u64);

    for j in (0..image_height).rev() {
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
