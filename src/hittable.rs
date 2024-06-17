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
    fn hit(&self, r: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> &AABB;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn HittableObject>>,
    aabb: AABB,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn HittableObject>) {
        // We need to make a copy of the bounding box here to make
        // the borrow checker happy.
        let object_aabb = object.bounding_box().clone();

        self.objects.push(object);
        self.aabb = AABB::combine_bounds(&self.aabb, &object_aabb);
    }

    pub fn objects(&self) -> Vec<Arc<dyn HittableObject>> {
        self.objects.clone()
    }
}

impl HittableObject for HittableList {
    fn hit(&self, ray: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
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

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}

pub struct Translate {
    object: Arc<dyn HittableObject>,
    offset: Vec3,
    aabb: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn HittableObject>, offset: Vec3) -> Self {
        let aabb = object.bounding_box().clone() + &offset;
        Self {
            object,
            offset,
            aabb,
        }
    }
}

impl HittableObject for Translate {
    fn hit(&self, ray: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

        if !self.object.hit(&offset_ray, range, record) {
            return false;
        }

        record.point = record.point + self.offset;

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}

pub struct RotateY {
    object: Arc<dyn HittableObject>,
    sin_theta: f64,
    cos_theta: f64,
    aabb: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn HittableObject>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let aabb = object.bounding_box().clone();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * aabb.x.end + (1 - i) as f64 * aabb.x.start;
                    let y = j as f64 * aabb.y.end + (1 - j) as f64 * aabb.y.start;
                    let z = k as f64 * aabb.z.end + (1 - k) as f64 * aabb.z.start;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let test_vec = Vec3::new(new_x, y, new_z);

                    min = vector::min_vec3(&min, &test_vec);
                    max = vector::max_vec3(&max, &test_vec);
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            aabb: AABB::from_points(&min, &max),
        }
    }
}

impl HittableObject for RotateY {
    fn hit(&self, ray: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin.x = self.cos_theta * ray.origin().x - self.sin_theta * ray.origin().z;
        origin.z = self.sin_theta * ray.origin().x + self.cos_theta * ray.origin().z;

        direction.x = self.cos_theta * ray.direction().x - self.sin_theta * ray.direction().z;
        direction.z = self.sin_theta * ray.direction().x + self.cos_theta * ray.direction().z;

        let rotated_ray = Ray::new(origin, direction, ray.time());

        if !self.object.hit(&rotated_ray, range, record) {
            return false;
        }

        let mut point = record.point;
        point.x = self.cos_theta * record.point.x + self.sin_theta * record.point.z;
        point.z = -self.sin_theta * record.point.x + self.cos_theta * record.point.z;
        record.point = point;

        let mut normal = record.normal;
        normal.x = self.cos_theta * record.normal.x + self.sin_theta * record.normal.z;
        normal.z = -self.sin_theta * record.normal.x + self.cos_theta * record.normal.z;
        record.normal = normal;

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}
