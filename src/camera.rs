//! A module to manage the camera

use crate::hittable::{HitRecord, HittableObject};
use crate::ray::Ray;
use crate::scene::Scene;

use rayon::prelude::*;

use glm;
use progressing::{mapping::Bar as MappingBar, Baring};
use rand::Rng;
use std::{ops::Range, sync::Mutex, time};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    position: glm::DVec3,
    max_depth: usize,
    pixel_samples_scale: f64,
    pixel00_location: glm::DVec3,
    pixel_delta_u: glm::DVec3,
    pixel_delta_v: glm::DVec3,
    defocus_angle: f64,
    defocus_disk_u: glm::DVec3,
    defocus_disk_v: glm::DVec3,
}

impl Camera {
    pub fn new(
        position: glm::DVec3,
        target_position: glm::DVec3,
        up_direction: glm::DVec3,
        aspect_ratio: f64,
        image_width: usize,
        fov: f64,
        defocus_angle: f64,
        focus_dist: f64,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        // Set the camer's image_height to an int no lower than 1
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let image_height = std::cmp::max(image_height, 1);

        // Set viewport dimensions
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = glm::normalize(position - target_position);
        let u = glm::normalize(glm::cross(up_direction, w));
        let v = glm::cross(w, u);

        // Create vectors to line the top and left borders
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Set the distance between the pixel centers in each direction
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Get the upper left corner in viewport space
        let viewport_upper_left =
            position - w * focus_dist - (viewport_u / 2.0) - (viewport_v / 2.0);

        // Set the top left pixel location
        let pixel00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            position,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn get_ray(self: &Self, x: f64, y: f64) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_location
            + (self.pixel_delta_u * (x + offset.x))
            + (self.pixel_delta_v * (y + offset.y));

        let ray_origin = match self.defocus_angle <= 0.0 {
            true => self.position,
            false => self.defocus_disk_sample(),
        };

        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    pub fn render(self: &Self, scene: &Scene) {
        eprintln!("Rendering scene...");

        let now = time::Instant::now();

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
                        self.ray_color(&r, &scene, self.max_depth)
                    })
                    // NOTE: swap these reduce calls to make it parallel
                    // .reduce(|| glm::dvec3(0.0, 0.0, 0.0), |acc, a| acc + a)
                    .reduce(|acc, a| acc + a)
                    .unwrap()
                    * self.pixel_samples_scale;

                color = glm::dvec3(
                    f64::max(color.x, 0.0).sqrt(),
                    f64::max(color.y, 0.0).sqrt(),
                    f64::max(color.z, 0.0).sqrt(),
                );

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

    pub fn ray_color(self: &Self, r: &Ray, scene: &Scene, depth: usize) -> glm::DVec3 {
        if depth <= 0 {
            return glm::dvec3(0.0, 0.0, 0.0);
        }

        let mut record = HitRecord::new(r.direction());

        let range = Range {
            start: 0.001,
            end: f64::INFINITY,
        };

        if scene.hit(r, &range, &mut record) {
            let mut scattered = Ray::new(glm::dvec3(0.0, 0.0, 0.0), glm::dvec3(0.0, 0.0, 0.0));
            let mut attenuation = glm::dvec3(0.0, 0.0, 0.0);
            let mat = record.mat.clone();

            if mat.scatter(&mut record, &mut attenuation, &mut scattered) {
                return self.ray_color(&scattered, scene, depth - 1) * attenuation;
            }

            return glm::dvec3(0.0, 0.0, 0.0);
        }

        // Sky background
        let unit_direction = glm::normalize(r.direction());
        let a = (unit_direction.y + 1.0) * 0.5;
        glm::dvec3(1.0, 1.0, 1.0) * (1.0 - a) + glm::dvec3(0.5, 0.7, 1.0) * a
    }

    fn defocus_disk_sample(self: &Self) -> glm::DVec3 {
        let p = Ray::random_unit_disk_vec();
        self.position + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }
}

fn sample_square() -> glm::DVec3 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();

    glm::dvec3(x - 0.5, y - 0.5, 0.0)
}
