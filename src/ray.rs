//! A definition for a ray

use glm;

pub struct Ray {
    origin: glm::DVec3,
    direction: glm::DVec3,
}

impl Ray {
    pub fn new(origin: glm::DVec3, direction: glm::DVec3) -> Ray {
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
