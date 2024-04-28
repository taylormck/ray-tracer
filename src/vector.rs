//! A wrapper around GLM vectors that make it a bit more convenient to use

use glm;
use rand::Rng;
use std::ops::Range;

pub type Vec3 = glm::DVec3;

pub type Color = glm::Vector3<u32>;

pub fn random_vec(range: Range<f64>) -> Vec3 {
    let mut rng = rand::thread_rng();

    glm::dvec3(
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
    )
}

pub fn random_sphere_vec() -> Vec3 {
    // TODO: there is almost certainly a better way to do this
    loop {
        let attempt = random_vec(-1.0..1.0);

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

pub fn random_unit_disk_vec() -> Vec3 {
    // TODO: there is almost certainly a better way to do this
    loop {
        let mut attempt = random_vec(-1.0..1.0);
        attempt.z = 0.0;

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

pub fn clamp_vec(v: &Vec3, range: Range<f64>) -> Vec3 {
    Vec3::new(
        v.x.clamp(range.start, range.end),
        v.y.clamp(range.start, range.end),
        v.z.clamp(range.start, range.end),
    )
}

pub fn zero_vec() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

pub fn one_vec() -> Vec3 {
    Vec3::new(1.0, 1.0, 1.0)
}

pub fn vec3_to_color(v: &Vec3) -> Color {
    Color::new(v.x as u32, v.y as u32, v.z as u32)
}

pub fn random_color_range(range: Range<u32>) -> Color {
    let mut rng = rand::thread_rng();

    Color {
        x: rng.gen_range(range.clone()),
        y: rng.gen_range(range.clone()),
        z: rng.gen_range(range.clone()),
    }
}

pub fn random_color() -> Color {
    random_color_range(0..255)
}
