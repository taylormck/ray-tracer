//! A simple raytracer in Rust
//! I'm building this both to practice Rust and to
//! brush up on graphics programming in general.
use glm;
use ray_tracer_rust::camera::Camera;
use ray_tracer_rust::scene::Scene;
use ray_tracer_rust::sphere::Sphere;
use std::sync::Arc;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 800;
const SAMPLES_PER_PIXEL: usize = 100;

fn main() {
    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL);

    let mut scene = Scene::new();
    scene.add(Arc::new(Sphere::new(glm::dvec3(0.0, 0.0, -1.0), 0.5)));
    scene.add(Arc::new(Sphere::new(glm::dvec3(1.5, 0.5, -2.0), 0.25)));
    scene.add(Arc::new(Sphere::new(glm::dvec3(0.0, -100.5, -1.0), 100.0)));

    camera.render(&scene);
}
