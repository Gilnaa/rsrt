#![feature(clamp)]

mod vec3;
mod hit;
mod material;

use vec3::{Vec3, Point3, Colour};
use hit::{HitRecord, Hit, HitList};
use material::{Material, Metal, Lambertian};
use std::sync::Arc;


const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 384;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

fn write_colour(colour: Colour, samples_per_pixel: usize) {
    let scale = 1.0 / samples_per_pixel as f32;
    let r = (colour.0 * scale).sqrt();
    let g = (colour.1 * scale).sqrt();
    let b = (colour.2 * scale).sqrt();

    let r = (255.999 * r.clamp(0.0, 0.999)) as u32;
    let g = (255.999 * g.clamp(0.0, 0.999)) as u32;
    let b = (255.999 * b.clamp(0.0, 0.999)) as u32;
    println!("{} {} {}", r, g, b);
}

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    fn colour(&self, world: &impl Hit, max_depth: usize) -> Colour {
        if max_depth == 0 {
            return Colour::ZERO;
        }

        if let Some(rec) = world.hit(self, 0.001, f32::INFINITY) {
            if let Some((attenuation, scattered)) = rec.material.scatter(self, &rec) {
                attenuation * scattered.colour(world, max_depth - 1)
            } else {
                Vec3::ZERO
            }
            // let target = rec.p + rec.normal + Vec3::random_unit_vector();
            // let target = rec.p + rec.normal.random_in_hemisphere();
            // let p = 0.5 * Ray {
            //     origin: rec.p,
            //     direction: target - rec.p
            // }.colour(world, max_depth - 1);
            //
            // Vec3(
            //     rec.shade.0 * p.0,
            //     rec.shade.1 * p.1,
            //     rec.shade.2 * p.2,
            // )
        } else {
            let unit = self.direction.unit();
            let t = 0.5 * (unit.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) +
                t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

struct Sphere {
    center: Point3,
    radius: f32,
    material: Arc<dyn Material + 'static + Sync>,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            let temp = if temp >= t_max || temp <= t_min {
                (-half_b + root) / a
            } else {
                temp
            };

            if temp < t_max && temp > t_min {
                let point = ray.at(temp);
                return Some(HitRecord::new(ray,
                                           point,
                                           (point - self.center) / self.radius,
                                           temp,
                                           self.material.clone()));
            }
        }

        None
    }
}


fn random_double() -> f32 {
    random_double_in_range(0.0, 1.0)
}

fn random_double_in_range(min: f32, max: f32) -> f32 {
    use rand::Rng;

    rand::thread_rng().gen_range(min, max)
}

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Vec3(0.0, 0.0, 0.0);
        let horizontal = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin
            - horizontal / 2.0f32
            - vertical / 2.0f32
            - Vec3(0.0, 0.0, FOCAL_LENGTH);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner +
                u * self.horizontal +
                v * self.vertical -
                self.origin
        }
    }
}

fn main() {
    // Header
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let camera = Camera::new();

    let world = {
        let mut world = HitList::new();

        let bg_material = Arc::new(Lambertian::new(Colour::UNIT / 2.0 + Colour::Y / 2.0));

        let default_material = Arc::new(Lambertian::new(Colour::X));

        let metal_material0 = Arc::new(Metal::new(Colour::UNIT * 0.8, 0.0));
        let metal_material1 = Arc::new(Metal::new(Colour::UNIT * 0.8, 0.3));
        let metal_material2 = Arc::new(Metal::new(Colour::UNIT * 0.8, 0.8));

        world.add(Sphere {
            center: Vec3(0f32, -100.5f32, -1f32),
            radius: 100.0,
            material: bg_material.clone(),
        });

        world.add(Sphere {
            center: Vec3(0f32, 0.2f32, -1.5f32),
            radius: 0.5,
            material: default_material.clone(),
        });

        world.add(Sphere {
            center: Vec3(0f32, 1.2f32, -1.5f32),
            radius: 0.5,
            material: metal_material0,
        });

        world.add(Sphere {
            center: Vec3(1f32, 0.2f32, -1.5f32),
            radius: 0.5,
            material: metal_material1,
        });

        world.add(Sphere {
            center: Vec3(-1f32, 0.2f32, -1.5f32),
            radius: 0.5,
            material: metal_material2,
        });

        world
    };

    const SAMPLES_PER_PIXELS: usize = 250;

    let mut frame = [[Colour::ZERO; IMAGE_WIDTH]; IMAGE_HEIGHT];

    // use rayon::prelude::*;
    // (&mut frame[..]).par_iter_mut().enumerate().for_each(|(idx, r)| {
    //     for i in 0..IMAGE_WIDTH {
    //         let mut pixel_color = Vec3::ZERO;
    //         for _ in 0..SAMPLES_PER_PIXELS {
    //             let u = (i as f32 + random_double()) / (IMAGE_WIDTH - 1) as f32;
    //             let v = (idx as f32 + random_double()) / (IMAGE_HEIGHT - 1) as f32;
    //             let r = camera.get_ray(u, v);
    //             pixel_color += r.colour(&world, 100);
    //         }
    //         r[i] = pixel_color;
    //     }
    // });

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::ZERO;
            for _ in 0..SAMPLES_PER_PIXELS {
                let u = (i as f32 + random_double()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random_double()) / (IMAGE_HEIGHT - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += r.colour(&world, 100);
            }

            frame[j][i] = pixel_color;
        }
    }

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            write_colour(frame[j][i], SAMPLES_PER_PIXELS);
        }
    }


}
