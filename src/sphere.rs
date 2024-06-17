//! A definition for a sphere

use crate::aabb::AABB;
use crate::hittable::{HitRecord, HittableObject};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Vec2, Vec3};
use glm;
use std::{
    f64::consts::{PI, TAU},
    ops::Range,
    sync::Arc,
};

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    velocity: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
    aabb: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, velocity: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        let end_point = center + velocity;

        let aabb = AABB::new(
            Range {
                start: f64::min(center.x - radius, center.x + radius),
                end: f64::max(end_point.x - radius, end_point.x + radius),
            },
            Range {
                start: f64::min(center.y - radius, center.y + radius),
                end: f64::max(end_point.y - radius, end_point.y + radius),
            },
            Range {
                start: f64::min(center.z - radius, center.z + radius),
                end: f64::max(end_point.z - radius, end_point.z + radius),
            },
        );

        Self {
            center,
            velocity,
            radius,
            material,
            aabb,
        }
    }

    fn get_uv(point: &Vec3) -> Vec2 {
        let y = -(point.y);
        let z = -(point.z);

        let theta = y.acos();
        let phi = z.atan2(point.x) + PI;

        Vec2::new(phi / TAU, theta / PI)
    }
}

impl HittableObject for Sphere {
    fn hit(&self, r: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
        let oc = (self.center + self.velocity * r.time()) - r.origin();
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
        record.mat = self.material.clone();

        let outward_normal = (record.point - self.center) / self.radius;
        record.set_normal(r, &outward_normal);
        record.uv = Self::get_uv(&outward_normal);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}
