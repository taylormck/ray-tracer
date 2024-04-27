//! A module to manage the camera

use crate::hittable::{HitRecord, HittableObject};
use crate::ray::Ray;
use crate::scene::Scene;

use rayon::prelude::*;

use glm;
use progressing::{mapping::Bar as MappingBar, Baring};
use rand::Rng;
use std::{ops::Range, sync::Mutex, time};

pub struct Camera {
    image_width: usize,
    image_height: usize,
    center: glm::DVec3,
    pixel00_location: glm::DVec3,
    pixel_delta_u: glm::DVec3,
    pixel_delta_v: glm::DVec3,
    samples_per_pixel: usize,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: usize) -> Self {
        // Set the camer's image_height to an int no lower than 1
        let image_height = (image_width as f64 / aspect_ratio).floor() as usize;
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

    pub fn render(self: &Self, scene: &Scene) {
        eprintln!("Rendering scene...");

        let now = time::Instant::now();

        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        let num_pixels: usize = self.image_height * self.image_width;

        let mut progress = MappingBar::with_range(0, self.image_height * self.image_width).timed();
        progress.set_len(20);

        let progress = Mutex::new(progress);

        let pixels: Vec<glm::Vector3<i32>> = (0..num_pixels)
            .into_par_iter()
            .map(|i| {
                let row_index = (i / self.image_width) as f64;
                let column_index = (i % self.image_width) as f64;

                let mut color = (0..self.samples_per_pixel)
                    // NOTE: uncomment this line to make it parallel
                    // .into_par_iter()
                    .map(|_| {
                        let r = self.get_ray(column_index, row_index);
                        self.ray_color(&r, &scene)
                    })
                    // NOTE: swap these reduce calls to make it parallel
                    // .reduce(|| glm::dvec3(0.0, 0.0, 0.0), |acc, a| acc + a)
                    .reduce(|acc, a| acc + a)
                    .unwrap()
                    * pixel_samples_scale;

                color = glm::dvec3(
                    color.x.clamp(0.0, 0.999),
                    color.y.clamp(0.0, 0.999),
                    color.z.clamp(0.0, 0.999),
                ) * 256.0;

                let color = glm::ivec3(color.x as i32, color.y as i32, color.z as i32);

                // Let's update the progress
                let mut progress = progress.lock().unwrap();
                progress.add(1_usize);

                if progress.has_progressed_significantly() {
                    progress.remember_significant_progress();
                    eprintln!("{}", progress);
                }

                color
            })
            .collect();

        eprintln!("Scene renderd in {}ms", now.elapsed().as_millis());

        eprintln!("Saving data to image...");
        let now = time::Instant::now();

        let mut progress = MappingBar::with_range(0, num_pixels).timed();
        progress.set_len(20);

        // Print the PPM header
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        // Print the PPM data
        for pixel in pixels {
            println!("{} {} {}", pixel.x, pixel.y, pixel.z);

            progress.add(1_usize);

            if progress.has_progressed_significantly() {
                progress.remember_significant_progress();
                eprintln!("{}", progress);
                eprintln!("{}", progress);
            }
        }

        eprintln!("Data saved to file in {}ms", now.elapsed().as_millis());
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
