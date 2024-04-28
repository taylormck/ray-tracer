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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: glm::DVec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: f64::min(fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        self: &Self,
        record: &mut HitRecord,
        attenuation: &mut glm::DVec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected_direction = glm::reflect(record.in_vec, record.normal);
        reflected_direction = glm::normalize(reflected_direction);
        reflected_direction = reflected_direction + Ray::random_unit_sphere_vec() * self.fuzz;

        *scattered = Ray::new(record.point, reflected_direction);
        *attenuation = self.albedo;

        glm::dot(reflected_direction, record.normal) > 0.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    albedo: glm::DVec3,
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(albedo: glm::DVec3, opacity: f64, refraction_index: f64) -> Self {
        Self {
            albedo: albedo * (1.0 - opacity),
            refraction_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        self: &Self,
        record: &mut HitRecord,
        attenuation: &mut glm::DVec3,
        scattered: &mut Ray,
    ) -> bool {
        let ri: f64 = match record.front_face {
            true => self.refraction_index.recip(),
            false => self.refraction_index,
        };

        let unit_direction = glm::normalize(record.in_vec);

        let cos_theta = f64::min(glm::dot(-unit_direction, record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

        let direction = match ri * sin_theta > 1.0 {
            true => glm::reflect(unit_direction, record.normal),
            false => glm::refract(unit_direction, record.normal, ri),
        };

        *scattered = Ray::new(record.point, direction);
        *attenuation = self.albedo;

        true
    }
}
