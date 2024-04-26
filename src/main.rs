//! A simple raytracer in Rust
//! I'm building this both to practice Rust and to
//! brush up on graphics programming in general.

mod hittable;
mod pixel;
mod ray;
mod scene;
mod sphere;

use glm;
use hittable::HitRecord;
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use std::{ops::Range, rc::Rc};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 800;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

fn ray_color(r: &Ray, scene: &Scene) -> glm::DVec3 {
    let mut record = HitRecord::new();
    let range = Range {
        start: 0.0,
        end: f64::INFINITY,
    };

    if scene.hit(r, &range, &mut record) {
        return (record.normal + glm::dvec3(1.0, 1.0, 1.0)) * 0.5;
    }

    // Sky background
    let unit_direction = glm::normalize(r.direction());
    let a = (unit_direction.y + 1.0) * 0.5;
    glm::dvec3(1.0, 1.0, 1.0) * (1.0 - a) + glm::dvec3(0.5, 0.7, 1.0) * a
}

fn main() {
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let camera_center: glm::DVec3 = glm::dvec3(0.0, 0.0, 0.0);
    let viewport_u = glm::dvec3(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = glm::dvec3(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    let viewport_upper_left = camera_center
        - glm::dvec3(0.0, 0.0, FOCAL_LENGTH)
        - (viewport_u / 2.0)
        - (viewport_v / 2.0);

    let pixel00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut scene = Scene::new();

    scene.add(Rc::new(Sphere::new(glm::dvec3(0.0, 0.0, -1.0), 0.5)));
    scene.add(Rc::new(Sphere::new(glm::dvec3(0.0, -100.5, -1.0), 100.0)));

    for j in 0..IMAGE_HEIGHT {
        // eprintln!("Scanlines remaining {}", IMAGE_HEIGHT - j);

        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_location + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);

            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let color = ray_color(&r, &scene);

            pixel::write_color(&color);
        }
    }

    eprintln!("Done rendering");
}
