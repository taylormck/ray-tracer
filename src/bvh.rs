//! BVH Tree

use crate::{
    aabb::AABB,
    hittable::{HitRecord, HittableObject},
    ray::Ray,
};

use std::{cmp::Ordering, ops::Range, sync::Arc};

pub struct BVHNode {
    left: Arc<dyn HittableObject>,
    right: Arc<dyn HittableObject>,
    aabb: AABB,
}

impl BVHNode {
    pub fn new(objects: &Vec<Arc<dyn HittableObject>>, start: usize, end: usize) -> Self {
        let mut aabb = AABB::default();

        for object in objects[start..end].iter() {
            aabb = AABB::combine_bounds(&aabb, object.bounding_box());
        }

        let axis = aabb.longest_axis_index();

        let length = end - start;

        let (left, right) = match length {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => (objects[start].clone(), objects[start + 1].clone()),
            _ => {
                objects.clone()[start..end].sort_by(|a, b| Self::box_compare(a, b, axis));

                let mid = start + length / 2;

                let left: Arc<dyn HittableObject> = Arc::new(Self::new(objects, start, mid));
                let right: Arc<dyn HittableObject> = Arc::new(Self::new(objects, mid, end));

                (left, right)
            }
        };

        let aabb = AABB::combine_bounds(left.bounding_box(), right.bounding_box());

        Self { left, right, aabb }
    }

    pub fn from(objects: Vec<Arc<dyn HittableObject>>) -> Self {
        let length = objects.len();
        Self::new(&objects, 0, length)
    }

    fn box_compare(
        a: &Arc<dyn HittableObject>,
        b: &Arc<dyn HittableObject>,
        axis: usize,
    ) -> Ordering {
        let a_axis = a.bounding_box().axis(axis).unwrap();
        let b_axis = b.bounding_box().axis(axis).unwrap();

        if a_axis.start < b_axis.start {
            Ordering::Less
        } else if b_axis.start < a_axis.start {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl HittableObject for BVHNode {
    fn hit(&self, ray: &Ray, range: &Range<f64>, record: &mut HitRecord) -> bool {
        if !self.aabb.hit(ray, range.clone()) {
            return false;
        }

        let hit_left = self.left.hit(ray, range, record);

        let range = match hit_left {
            true => range.start..record.t,
            false => range.clone(),
        };

        let hit_right = self.right.hit(ray, &range, record);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}
