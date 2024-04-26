//! A definition for a sphere

use crate::hittable::{HitRecord, HittableObject};
use crate::ray::Ray;
use glm;
use std::ops::Range;

pub struct Sphere {
    center: glm::DVec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: glm::DVec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl HittableObject for Sphere {
    fn hit(self: &Self, r: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = glm::ext::sqlength(r.direction());
        let h = glm::dot(r.direction(), oc);
        let c = glm::ext::sqlength(oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (h - sqrt_discriminant) / a;

        if !range.contains(&root) {
            root = (h + sqrt_discriminant) / a;

            if !range.contains(&root) {
                return false;
            }
        }

        record.t = root;
        record.point = r.at(record.t);

        let outward_normal = (record.point - self.center) / self.radius;
        record.set_normal(r, &outward_normal);

        true
    }
}
