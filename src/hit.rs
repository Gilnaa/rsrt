use super::Ray;
use super::{Vec3, Point3, Colour};
use crate::Material;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material + Sync>,
}

impl HitRecord {
    pub fn new(ray: &Ray,
               p: Point3,
               outward_normal: Vec3,
               t: f32,
               material: Arc<dyn Material + Sync>) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitList(Vec<Box<dyn Hit + Sync + 'static>>);

impl HitList {
    pub fn new() -> Self {
        HitList(Vec::new())
    }

    pub fn add<H: Hit + 'static + Sync>(&mut self, obj: H) {
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