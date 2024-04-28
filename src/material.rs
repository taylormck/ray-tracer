use crate::hittable::HitRecord;
use crate::ray::Ray;
use glm;

pub trait Material: Send + Sync {
    fn scatter(
        self: &Self,
        record: &mut HitRecord,
        attenuation: &mut glm::DVec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct DebugMaterial;

impl Material for DebugMaterial {
    fn scatter(
        self: &Self,
        _record: &mut HitRecord,
        _attenuation: &mut glm::DVec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: glm::DVec3,
}

impl Lambertian {
    pub fn new(albedo: glm::DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        self: &Self,
        record: &mut HitRecord,
        attenuation: &mut glm::DVec3,
        scattered: &mut Ray,
    ) -> bool {
        // NOTE: We choose to always scatter here
        // We may want to change the material to absorb some amount
        // of the incoming light.
        let mut scatter_direction = record.normal + Ray::random_unit_sphere_vec();

        if Ray::is_vec_near_zero(&scatter_direction) {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(record.point, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: glm::DVec3,
}

impl Metal {
    pub fn new(albedo: glm::DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        self: &Self,
        record: &mut HitRecord,
        attenuation: &mut glm::DVec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected_direction = glm::reflect(record.in_vec, record.normal);
        *scattered = Ray::new(record.point, reflected_direction);
        *attenuation = self.albedo;
        true
    }
}
