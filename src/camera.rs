//! A module to manage the camera

use crate::hittable::{HitRecord, HittableObject};
use crate::pixel;
use crate::ray::Ray;
use crate::scene::Scene;
use glm;
use std::ops::Range;

pub struct Camera {
    _aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: glm::DVec3,
    pixel00_location: glm::DVec3,
    pixel_delta_u: glm::DVec3,
    pixel_delta_v: glm::DVec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
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
        }
    }

    pub fn render(self: &Self, scene: Scene) {
        // Print the PPM header
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            // eprintln!("Scanlines remaining {}", IMAGE_HEIGHT - j);

            for i in 0..self.image_width {
                let j = j as f64;
                let i = i as f64;

                let pixel_center =
                    self.pixel00_location + self.pixel_delta_u * i + self.pixel_delta_v * j;

                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let color = self.ray_color(&r, &scene);

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