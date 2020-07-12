use crate::{Ray, HitRecord, Vec3, Colour};

pub trait Material: Send {
    fn scatter (&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Colour, Ray)>;
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Colour, Ray)> {
        let scatter_direction = hit_rec.normal + Vec3::random_unit_vector();
        Some((
            self.albedo,
            Ray{origin: hit_rec.p, direction: scatter_direction},
        ))
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Colour,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Colour, Ray)> {
        fn reflect(v: Vec3, n: Vec3) -> Vec3 {
            v - 2.0 * v.dot(n) * n
        }

        let reflected = reflect(ray.direction.unit(), hit_rec.normal);
        if reflected.dot(hit_rec.normal) > 0.0 {
            let scattered = Ray {
                origin: hit_rec.p,
                direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            };
            return Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
