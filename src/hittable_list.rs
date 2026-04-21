use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}
// Create our Implementation for HittableList
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

//Create implementation of Hittable for HittableList
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            // If the ray hits this specific object...
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                // Update our closest boundary so we ignore objects behind it
                closest_so_far = rec.t;
                // Save the hit record
                hit_anything = Some(rec);
            }
        }
        hit_anything
    }
}
