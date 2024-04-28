//! A simple raytracer in Rust
//! I'm building this both to practice Rust and to
//! brush up on graphics programming in general.
use glm;
use ray_tracer_rust::camera::Camera;
use ray_tracer_rust::material::{refraction_indices, Dielectric, Lambertian, Metal};
use ray_tracer_rust::scene::Scene;
use ray_tracer_rust::sphere::Sphere;
use std::sync::Arc;

const CAMERA_POSITION: glm::DVec3 = glm::DVec3 {
    x: -1.0,
    y: 1.0,
    z: 0.5,
};

const CAMERA_TARGET: glm::DVec3 = glm::DVec3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};

const CAMERA_UP: glm::DVec3 = glm::DVec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 800;
const SAMPLES_PER_PIXEL: usize = 10;
const MAX_DEPTH: usize = 10;
const FOV: f64 = 75.0;

fn main() {
    let camera = Camera::new(
        CAMERA_POSITION,
        CAMERA_TARGET,
        CAMERA_UP,
        ASPECT_RATIO,
        IMAGE_WIDTH,
        FOV,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
    );

    let mut scene = Scene::new();

    let material_ground = Arc::new(Lambertian::new(glm::dvec3(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(glm::dvec3(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(glm::dvec3(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(glm::dvec3(0.8, 0.6, 0.2), 1.0));

    let material_glass_blue = Arc::new(Dielectric::new(
        glm::dvec3(0.2, 0.4, 0.9),
        0.2,
        refraction_indices::GLASS / refraction_indices::WATER,
    ));

    let material_glass_red = Arc::new(Dielectric::new(
        glm::dvec3(0.8, 0.2, 0.1),
        0.5,
        refraction_indices::AIR / refraction_indices::WATER,
    ));

    let material_glass_clear = Arc::new(Dielectric::new(
        glm::dvec3(1.0, 1.0, 1.0),
        0.0,
        refraction_indices::GLASS,
    ));

    let material_glass_clear_inner = Arc::new(Dielectric::new(
        glm::dvec3(1.0, 1.0, 1.0),
        0.0,
        refraction_indices::AIR / refraction_indices::GLASS,
    ));

    // Ground
    scene.add(Arc::new(Sphere::new(
        glm::dvec3(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    // Central ball
    scene.add(Arc::new(Sphere::new(
        glm::dvec3(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    // Right ball
    scene.add(Arc::new(Sphere::new(
        glm::dvec3(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Left ball
    scene.add(Arc::new(Sphere::new(
        glm::dvec3(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));

    scene.add(Arc::new(Sphere::new(
        glm::dvec3(0.25, -0.15, -0.4),
        0.2,
        material_glass_blue,
    )));

    scene.add(Arc::new(Sphere::new(
        glm::dvec3(-0.25, -0.1, -0.4),
        0.1,
        material_glass_clear,
    )));

    scene.add(Arc::new(Sphere::new(
        glm::dvec3(-0.25, -0.1, -0.4),
        0.07,
        material_glass_clear_inner,
    )));

    scene.add(Arc::new(Sphere::new(
        glm::dvec3(0.2, 0.3, -0.5),
        0.2,
        material_glass_red,
    )));

    camera.render(&scene);
}
