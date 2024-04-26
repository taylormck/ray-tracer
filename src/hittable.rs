//! A struct to describe hittable objects in a scene

use crate::ray::Ray;
use glm;
use std::ops::Range;

#[derive(Clone)]
pub struct HitRecord {
    pub point: glm::DVec3,
    pub normal: glm::DVec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: glm::dvec3(0.0, 0.0, 0.0),
            normal: glm::dvec3(0.0, 0.0, 0.0),
            t: 0.0,
        }
    }

    pub fn set_normal(&mut self, ray: &Ray, outward_normal: &glm::DVec3) {
        let front_face = glm::dot(ray.direction(), *outward_normal) < 0.0;

        self.normal = match front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}

pub trait HittableObject {
    fn hit(self: &Self, r: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool;
}
