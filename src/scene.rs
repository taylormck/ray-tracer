//! A definition for a scene full of objects to render

use crate::hittable::{HitRecord, HittableObject};
use crate::ray::Ray;
use std::{ops::Range, rc::Rc};

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
}

impl HittableObject for Scene {
    fn hit(self: &Self, ray: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut range = range.clone();

        for object in self.objects.iter() {
            if object.hit(ray, &range, &mut temp_record) {
                hit_anything = true;
                range.end = temp_record.t;

                // TODO: could this be moved out of the loop?
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}
