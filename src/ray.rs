//! A definition for a ray

use glm;
use rand::Rng;
use std::ops::Range;

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

    pub fn random_vec(range: Range<f64>) -> glm::DVec3 {
        let mut rng = rand::thread_rng();

        glm::dvec3(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
        )
    }

    pub fn random_unit_sphere_vec() -> glm::DVec3 {
        // TODO: there is almost certainly a better way to do this
        loop {
            let attempt = Self::random_vec(-1.0..1.0);

            if glm::ext::sqlength(attempt) < 1.0 {
                return glm::normalize(attempt);
            }
        }
    }

    pub fn random_hemisphere_vec(normal: glm::DVec3) -> glm::DVec3 {
        let mut unit_sphere_vec = Self::random_unit_sphere_vec();

        if glm::dot(unit_sphere_vec, normal) < 0.0 {
            unit_sphere_vec = -unit_sphere_vec;
        }

        unit_sphere_vec
    }
}
