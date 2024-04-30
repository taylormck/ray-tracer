//! A struct to describe hittable objects in a scene

use crate::aabb::AABB;
use crate::material::{DebugMaterial, Material};
use crate::ray::Ray;
use crate::vector;
use crate::vector::{Vec2, Vec3};
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
    pub uv: Vec2,
}

impl HitRecord {
    pub fn new(in_ray: &Ray) -> Self {
        Self {
            in_ray: *in_ray,
            point: vector::zero_vec3(),
            normal: vector::zero_vec3(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(DebugMaterial),
            uv: Vec2::new(0.0, 0.0),
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

pub struct HittableList {
    objects: Vec<Arc<dyn HittableObject>>,
    aabb: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            aabb: AABB::new_empty(),
        }
    }

    pub fn add(self: &mut Self, object: Arc<dyn HittableObject>) {
        // We need to make a copy of the bounding box here to make
        // the borrow checker happy.
        let object_aabb = object.bounding_box().clone();

        self.objects.push(object);
        self.aabb = AABB::combine_bounds(&self.aabb, &object_aabb);
    }

    pub fn objects(self: &Self) -> Vec<Arc<dyn HittableObject>> {
        self.objects.clone()
    }
}

impl HittableObject for HittableList {
    fn hit(self: &Self, ray: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new(ray);
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

    fn bounding_box(self: &Self) -> &AABB {
        &self.aabb
    }
}
