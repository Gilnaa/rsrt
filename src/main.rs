#![feature(clamp)]

mod vec3;
use vec3::Vec3;

type Point3 = Vec3;
type Colour = Vec3;


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

struct Ray {
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
            // let target = rec.p + rec.normal + Vec3::random_unit_vector();
            let target = rec.p + rec.normal.random_in_hemisphere();
            let p = 0.5 * Ray {
                origin: rec.p,
                direction: target - rec.p
            }.colour(world, max_depth - 1);

            Vec3(
                rec.shade.0 * p.0,
                rec.shade.1 * p.1,
                rec.shade.2 * p.2,
            )
        } else {
            let unit = self.direction.unit();
            let t = 0.5 * (unit.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) +
                t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    shade: Colour,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Point3, outward_normal: Vec3, t: f32, shade: Colour) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        HitRecord {
            p,
            normal,
            t,
            front_face,
            shade,
        }
    }
}

trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

struct Sphere {
    center: Point3,
    radius: f32,
    shade: Colour,
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
                                           self.shade));
            }
        }

        None
    }
}

struct HitList(Vec<Box<dyn Hit>>);

impl HitList {
    pub fn new() -> Self {
        HitList(Vec::new())
    }

    pub fn add<H: Hit + 'static>(&mut self, obj: H) {
        self.0.push(Box::new(obj));
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Hit for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record = None;

        for o in self.0.iter() {
            if let Some(new_rec) = o.hit(ray, t_min, closest_so_far) {
                closest_so_far = new_rec.t;
                record.replace(new_rec);
            }
        }

        record
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

        world.add(Sphere {
            center: Vec3(0f32, 0f32, -1f32),
            radius: 0.5,
            shade: Vec3::X,
        });

        world.add(Sphere {
            center: Vec3(0f32, -100.5f32, -1f32),
            radius: 100.0,
            shade: Vec3::Y / 2.0 + Vec3::Z,
        });

        world
    };

    const SAMPLES_PER_PIXELS: usize = 100;

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::ZERO;
            for _ in 0..SAMPLES_PER_PIXELS {
                let u = (i as f32 + random_double()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random_double()) / (IMAGE_HEIGHT - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += r.colour(&world, 100000);
            }

            write_colour(pixel_color, SAMPLES_PER_PIXELS);
        }
    }
}
