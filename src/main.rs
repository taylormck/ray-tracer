//! A simple raytracer in Rust
//! I'm building this both to practice Rust and to
//! brush up on graphics programming in general.

mod camera;
mod hittable;
mod pixel;
mod ray;
mod scene;
mod sphere;

use camera::Camera;
use glm;
use scene::Scene;
use sphere::Sphere;
use std::rc::Rc;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 800;

fn main() {
    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH);

    let mut scene = Scene::new();
    scene.add(Rc::new(Sphere::new(glm::dvec3(0.0, 0.0, -1.0), 0.5)));
    scene.add(Rc::new(Sphere::new(glm::dvec3(1.5, 0.5, -2.0), 0.25)));
    scene.add(Rc::new(Sphere::new(glm::dvec3(0.0, -100.5, -1.0), 100.0)));

    camera.render(scene);
}
