//! A definition for a sphere

use crate::hittable::{HitRecord, HittableObject};
use crate::ray::Ray;
use glm;

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
    fn hit(self: &Self, r: &Ray, ray_t_min: f64, ray_t_max: f64, record: &mut HitRecord) -> bool {
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

        if root <= ray_t_min || root >= ray_t_max {
            root = (h + sqrt_discriminant) / a;

            if root <= ray_t_min || root >= ray_t_max {
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
