//! A definition for a scene full of objects to render

use crate::hittable::{HitRecord, HittableObject};
use crate::ray::Ray;
use std::rc::Rc;

pub struct Scene {
    objects: Vec<Rc<dyn HittableObject>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(self: &mut Self, object: Rc<dyn HittableObject>) {
        self.objects.push(object);
    }

    pub fn hit(
        self: &Self,
        ray: &Ray,
        ray_t_min: f64,
        ray_t_max: f64,
        record: &mut HitRecord,
    ) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t_max;

        for object in self.objects.iter() {
            if object.hit(ray, ray_t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;

                // TODO: could this be moved out of the loop?
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}
