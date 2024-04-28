//! A definition for a ray

use crate::vector::Vec3;
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: glm::DVec3, direction: Vec3) -> Ray {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> glm::DVec3 {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> glm::DVec3 {
        self.origin
    }

    pub fn direction(&self) -> glm::DVec3 {
        self.direction
    }
}
