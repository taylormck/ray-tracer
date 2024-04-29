use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vector;
use crate::vector::Color;
use rand::Rng;
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(
        self: &Self,
        record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct DebugMaterial;

impl Material for DebugMaterial {
    fn scatter(
        self: &Self,
        _record: &mut HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self {
            texture: texture.clone(),
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self::from_texture(Arc::new(SolidColor::from(color)))
    }

    pub fn from_color_components(r: f64, g: f64, b: f64) -> Self {
        Self::from_color(Color::new(r, g, b))
    }
}

impl Material for Lambertian {
    fn scatter(
        self: &Self,
        record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // NOTE: We choose to always scatter here
        // We may want to change the material to absorb some amount
        // of the incoming light.
        let mut scatter_direction = record.normal + vector::random_unit_sphere_vec();

        if vector::is_vec_near_zero(&scatter_direction) {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(record.point, scatter_direction, record.in_ray.time());
        *attenuation = self.texture.sample(&record.uv, &record.point);

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
        let mut reflected_direction = glm::reflect(record.in_ray.direction(), record.normal);
        reflected_direction = glm::normalize(reflected_direction);
        reflected_direction = reflected_direction + vector::random_unit_sphere_vec() * self.fuzz;

        *scattered = Ray::new(record.point, reflected_direction, record.in_ray.time());
        *attenuation = self.albedo;

        glm::dot(reflected_direction, record.normal) > 0.0
    }
}

pub mod refraction_indices {
    pub const AIR: f64 = 1.0;
    pub const GLASS: f64 = 1.5;
    pub const WATER: f64 = 1.33;
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

    fn reflectance(self: &Self, cosine: f64) -> f64 {
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        self: &Self,
        record: &mut HitRecord,
        attenuation: &mut glm::DVec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut rng = rand::thread_rng();

        let ri: f64 = match record.front_face {
            true => self.refraction_index.recip(),
            false => self.refraction_index,
        };

        let unit_direction = glm::normalize(record.in_ray.direction());

        let cos_theta = f64::min(glm::dot(-unit_direction, record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

        let should_reflect = ri * sin_theta > 1.0 || self.reflectance(cos_theta) > rng.gen();

        let direction = match should_reflect {
            true => glm::reflect(unit_direction, record.normal),
            false => glm::refract(unit_direction, record.normal, ri),
        };

        *scattered = Ray::new(record.point, direction, record.in_ray.time());

        // TODO: This will need some re-working.
        // The attunuation should scale with how much time the light spends inside the glass.
        *attenuation = self.albedo;

        true
    }
}
