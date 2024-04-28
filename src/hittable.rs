//! A struct to describe hittable objects in a scene

use crate::aabb::AABB;
use crate::material::{DebugMaterial, Material};
use crate::ray::Ray;
use crate::vector;
use crate::vector::Vec3;
use glm;
use std::{ops::Range, sync::Arc};

#[derive(Clone)]
pub struct HitRecord {
    pub in_ray: Ray,
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(in_ray: &Ray) -> Self {
        Self {
            in_ray: *in_ray,
            point: vector::zero_vec(),
            normal: vector::zero_vec(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(DebugMaterial),
        }
    }

    pub fn set_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = glm::dot(ray.direction(), *outward_normal) < 0.0;

        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}

pub trait HittableObject: Send + Sync {
    fn hit(self: &Self, r: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool;
    fn bounding_box(self: &Self) -> &AABB;
}
