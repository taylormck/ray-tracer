//! A wrapper around GLM vectors that make it a bit more convenient to use

use glm;
use rand::Rng;
use std::ops::Range;

pub type Vec2 = glm::DVec2;
pub type Vec3 = glm::DVec3;
pub type Color = glm::DVec3;
pub type Pixel = glm::Vector3<u32>;

pub fn random_vec2(range: Range<f64>) -> Vec2 {
    let mut rng = rand::thread_rng();

    Vec2::new(rng.gen_range(range.clone()), rng.gen_range(range.clone()))
}

pub fn random_vec3(range: Range<f64>) -> Vec3 {
    let mut rng = rand::thread_rng();

    Vec3::new(
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
    )
}

pub fn random_unit_vec3() -> Vec3 {
    glm::normalize(random_vec3(0.0..1.0))
}

pub fn random_sphere_vec() -> Vec3 {
    // TODO: there is almost certainly a better way to do this
    // This is "Las Vegas" sampling.
    // Look into "Monte Carlo" sampling.
    loop {
        let attempt = random_vec3(-1.0..1.0);

        if glm::ext::sqlength(attempt) < 1.0 {
            return attempt;
        }
    }
}

pub fn random_unit_sphere_vec() -> Vec3 {
    glm::normalize(random_sphere_vec())
}

pub fn random_hemisphere_vec(normal: glm::DVec3) -> Vec3 {
    let mut sphere_vec = random_sphere_vec();

    if glm::dot(sphere_vec, normal) < 0.0 {
        sphere_vec = -sphere_vec;
    }

    sphere_vec
}

pub fn random_unit_hemisphere_vec(normal: glm::DVec3) -> Vec3 {
    glm::normalize(random_hemisphere_vec(normal))
}

pub fn random_unit_disk_vec() -> Vec2 {
    // TODO: there is almost certainly a better way to do this
    // This is "Las Vegas" sampling.
    // Look into "Monte Carlo" sampling.
    loop {
        let attempt = random_vec2(-1.0..1.0);

        if glm::ext::sqlength(attempt) < 1.0 {
            return attempt;
        }
    }
}

pub fn random_unit_square_vec() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();

    Vec3::new(x - 0.5, y - 0.5, 0.0)
}

pub fn is_vec_near_zero(v: &Vec3) -> bool {
    static EPSILON: f64 = 0.00000001;
    static RANGE: Range<f64> = 0.0..EPSILON;
    RANGE.contains(&v.x) && RANGE.contains(&v.y) && RANGE.contains(&v.z)
}

pub fn sqrt_vec(v: &Vec3) -> Vec3 {
    Vec3::new(
        f64::max(v.x, 0.0).sqrt(),
        f64::max(v.y, 0.0).sqrt(),
        f64::max(v.z, 0.0).sqrt(),
    )
}

pub fn clamp_vec2(v: &Vec2, range: Range<f64>) -> Vec2 {
    Vec2::new(
        v.x.clamp(range.start, range.end),
        v.y.clamp(range.start, range.end),
    )
}

pub fn clamp_vec3(v: &Vec3, range: Range<f64>) -> Vec3 {
    Vec3::new(
        v.x.clamp(range.start, range.end),
        v.y.clamp(range.start, range.end),
        v.z.clamp(range.start, range.end),
    )
}

pub fn zero_vec3() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

pub fn one_vec3() -> Vec3 {
    Vec3::new(1.0, 1.0, 1.0)
}

pub fn up_vec3() -> Vec3 {
    Vec3::new(0.0, 1.0, 0.0)
}

pub fn color_to_pixel(v: &Color) -> Pixel {
    Pixel::new(v.x as u32, v.y as u32, v.z as u32)
}

pub fn random_color_range(range: Range<u32>) -> Pixel {
    let mut rng = rand::thread_rng();

    Pixel {
        x: rng.gen_range(range.clone()),
        y: rng.gen_range(range.clone()),
        z: rng.gen_range(range.clone()),
    }
}

pub fn random_color() -> Pixel {
    random_color_range(0..255)
}

pub fn min_vec3(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(f64::min(a.x, b.x), f64::min(a.y, b.y), f64::min(a.z, b.z))
}

pub fn max_vec3(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(f64::max(a.x, b.x), f64::max(a.y, b.y), f64::max(a.z, b.z))
}
