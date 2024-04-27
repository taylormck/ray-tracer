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
        let c = glm::ext::sqlength(oc) - (self.radius * self.radius);

        let discriminant = h * h - a * c;

        // If the discriminant is 0, the ray touches the sphere once.
        // If the discriminant is positive, the ray passes through the
        // sphere, touching the surface twice.
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Try the closer intersection first
        let mut root = (h - sqrt_discriminant) / a;

        if !range.contains(&root) {
            // If the closer intersection wasn't in our target range,
            // check the further intersection.
            root = (h + sqrt_discriminant) / a;

            if !range.contains(&root) {
                // If neither intersection is in our range (say, if the sphere
                // is simply behind the camera), then just return false.
                return false;
            }
        }

        // Reassigning root to make it immutable
        let root = root;

        record.t = root;
        record.point = r.at(record.t);

        let outward_normal = (record.point - self.center) / self.radius;
        record.set_normal(r, &outward_normal);

        true
    }
}
