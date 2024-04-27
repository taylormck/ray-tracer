//! A module to manage the camera

use crate::hittable::{HitRecord, HittableObject};
use crate::pixel;
use crate::ray::Ray;
use crate::scene::Scene;
use glm;
use rand::Rng;
use std::ops::Range;
pub struct Camera {
    _aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: glm::DVec3,
    pixel00_location: glm::DVec3,
    pixel_delta_u: glm::DVec3,
    pixel_delta_v: glm::DVec3,
    samples_per_pixel: i32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        // Set the camer's image_height to an int no lower than 1
        let image_height = (image_width as f64 / aspect_ratio).floor() as i32;
        let image_height = std::cmp::max(image_height, 1);

        let center = glm::dvec3(0.0, 0.0, 0.0);

        // Set viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Create vectors to line the top and left borders
        let viewport_u = glm::dvec3(viewport_width, 0.0, 0.0);
        let viewport_v = glm::dvec3(0.0, -viewport_height, 0.0);

        // Set the distance between the pixel centers in each direction
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Get the upper left corner in viewport space
        let viewport_upper_left =
            center - glm::dvec3(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

        // Set the top left pixel location
        let pixel00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            _aspect_ratio: aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }

    fn get_ray(self: &Self, x: f64, y: f64) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_location
            + (self.pixel_delta_u * (x + offset.x))
            + (self.pixel_delta_v * (y + offset.y));

        Ray::new(self.center, pixel_sample - self.center)
    }

    pub fn render(self: &Self, scene: Scene) {
        // Print the PPM header
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        eprintln!("Rendering...");

        for j in 0..self.image_height {
            // eprintln!("Scanlines remaining {}", IMAGE_HEIGHT - j);

            for i in 0..self.image_width {
                let j = j as f64;
                let i = i as f64;

                let mut color = glm::dvec3(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color = color + self.ray_color(&r, &scene);
                }

                let color = color * pixel_samples_scale;

                pixel::write_color(&color);
            }
        }

        eprintln!("Done rendering");
    }

    pub fn ray_color(self: &Self, r: &Ray, scene: &Scene) -> glm::DVec3 {
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
}

fn sample_square() -> glm::DVec3 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();

    glm::dvec3(x - 0.5, y - 0.5, 0.0)
}
