//! A struct to describe hittable objects in a scene

use crate::material::{DebugMaterial, Material};
use crate::ray::Ray;
use glm;
use std::{ops::Range, sync::Arc};

#[derive(Clone)]
pub struct HitRecord {
    pub in_vec: glm::DVec3,
    pub point: glm::DVec3,
    pub normal: glm::DVec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(in_vec: glm::DVec3) -> Self {
        Self {
            in_vec,
            point: glm::dvec3(0.0, 0.0, 0.0),
            normal: glm::dvec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Arc::new(DebugMaterial),
        }
    }

    pub fn set_normal(&mut self, ray: &Ray, outward_normal: &glm::DVec3) {
        self.front_face = glm::dot(ray.direction(), *outward_normal) < 0.0;

        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}

pub trait HittableObject: Send + Sync {
    fn hit(self: &Self, r: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool;
}
