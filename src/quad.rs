//! Quads

use crate::{
    aabb::AABB,
    hittable::{HitRecord, HittableList, HittableObject},
    material::Material,
    ray::Ray,
    vector,
    vector::{Vec2, Vec3},
};
use glm;
use std::{ops::Range, sync::Arc};

const EPSILON: f64 = 1e-8;

#[derive(Clone)]
pub struct Quad {
    point: Vec3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Material>,
    aabb: AABB,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(point: Vec3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let n = glm::cross(u, v);
        let normal = glm::normalize(n);

        Self {
            point,
            u,
            v,
            material,
            aabb: create_bounding_box(&point, &u, &v),
            normal,
            d: glm::dot(normal, point),
            w: n / glm::ext::sqlength(n),
        }
    }

    pub fn box_from_opposite_corners(
        a: Vec3,
        b: Vec3,
        material: Arc<dyn Material>,
    ) -> Arc<dyn HittableObject> {
        let mut sides = HittableList::default();

        let min = vector::min_vec3(&a, &b);
        let max = vector::max_vec3(&a, &b);
        let diff = max - min;

        let dx = Vec3::new(diff.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, diff.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, diff.z);

        sides.add(Arc::new(Self::new(
            Vec3::new(min.x, min.y, max.z),
            dx,
            dy,
            material.clone(),
        )));

        sides.add(Arc::new(Self::new(
            Vec3::new(max.x, min.y, max.z),
            -dz,
            dy,
            material.clone(),
        )));

        sides.add(Arc::new(Self::new(
            Vec3::new(max.x, min.y, min.z),
            -dx,
            dy,
            material.clone(),
        )));

        sides.add(Arc::new(Self::new(
            Vec3::new(min.x, min.y, min.z),
            dz,
            dy,
            material.clone(),
        )));

        sides.add(Arc::new(Self::new(
            Vec3::new(min.x, max.y, max.z),
            dx,
            -dz,
            material.clone(),
        )));

        sides.add(Arc::new(Self::new(
            Vec3::new(min.x, min.y, min.z),
            dx,
            dz,
            material,
        )));

        Arc::new(sides)
    }
}

impl HittableObject for Quad {
    fn hit(&self, ray: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
        let denominator = glm::dot(self.normal, ray.direction());

        if f64::abs(denominator) < EPSILON {
            return false;
        }

        let t = (self.d - glm::dot(self.normal, ray.origin())) / denominator;

        if !range.contains(&t) {
            return false;
        }

        let intersection = ray.at(t);

        let planar_hit_point_vector = intersection - self.point;
        let alpha = glm::dot(self.w, glm::cross(planar_hit_point_vector, self.v));
        let beta = glm::dot(self.w, glm::cross(self.u, planar_hit_point_vector));

        if !(0.0..1.0).contains(&alpha) || !(0.0..1.0).contains(&beta) {
            return false;
        }

        record.t = t;
        record.point = intersection;
        record.mat = self.material.clone();
        record.set_normal(ray, &self.normal);
        record.uv = Vec2::new(alpha, beta);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}

fn create_bounding_box(point: &Vec3, u: &Vec3, v: &Vec3) -> AABB {
    let pu = *point + *u;
    let pv = *point + *v;
    let puv = pu + *v;

    let diagonal_1 = AABB::from_points(point, &puv);
    let diagonal_2 = AABB::from_points(&pu, &pv);

    AABB::combine_bounds(&diagonal_1, &diagonal_2)
}
