//! A hittable object that fills a space and proportionally displaces rays

use crate::{
    hittable::{HitRecord, HittableObject},
    material::{Isotropic, Material},
    ray::Ray,
    texture::Texture,
    vector::{Color, Vec3},
};

use glm;
use rand::Rng;
use std::{ops::Range, sync::Arc};

const EPSILON: f64 = 0.00001;

pub struct ConstantMedium {
    boundary: Arc<dyn HittableObject>,
    negative_inverse_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn HittableObject>, density: f64, texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            negative_inverse_density: -density.recip(),
            phase_function: Arc::new(Isotropic::from_texture(texture)),
        }
    }

    pub fn from_color(boundary: Arc<dyn HittableObject>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            negative_inverse_density: -density.recip(),
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl HittableObject for ConstantMedium {
    fn hit(&self, ray: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
        let mut rng = rand::thread_rng();
        let mut record1 = HitRecord::new(ray);
        let mut record2 = HitRecord::new(ray);

        let universe = f64::NEG_INFINITY..f64::INFINITY;

        if !self.boundary.hit(ray, &universe, &mut record1) {
            return false;
        }

        let range_after_entrance = (record1.t + EPSILON)..f64::INFINITY;

        if !self.boundary.hit(ray, &range_after_entrance, &mut record2) {
            return false;
        }

        record1.t = f64::max(record1.t, range.start);
        record2.t = f64::min(record2.t, range.end);

        if record1.t >= record2.t {
            return false;
        }

        record1.t = f64::max(record1.t, 0.0);

        let ray_length = glm::length(ray.direction());
        let distance_inside_boundary = (record2.t - record1.t) * ray_length;

        let hit_distance_coefficient = rng.gen::<f64>().ln();
        let hit_distance = self.negative_inverse_density * hit_distance_coefficient;

        if hit_distance > distance_inside_boundary {
            return false;
        }

        record.t = record1.t + hit_distance / ray_length;
        record.point = ray.at(record.t);
        record.mat = self.phase_function.clone();

        // Both of these are completely arbitrary
        record.normal = Vec3::new(1.0, 0.0, 0.0);
        record.front_face = true;

        true
    }

    fn bounding_box(self: &Self) -> &crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}
