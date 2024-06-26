//! A module to manage the camera

use crate::hittable::{HitRecord, HittableObject};
use crate::ray::Ray;
use crate::vector;
use crate::vector::{Color, Pixel, Vec3};

use rayon::prelude::*;

use progressing::{mapping::Bar as MappingBar, Baring};
use rand::Rng;
use std::{sync::Mutex, time};

#[derive(Debug, Copy, Clone)]
pub struct CameraSettings {
    pub position: Vec3,
    pub target_position: Vec3,
    pub up_direction: Vec3,
    pub fov: f64,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub aspect_ratio: f64,
    pub background_color: Color,
}

#[derive(Debug, Copy, Clone)]
pub struct RenderSettings {
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    position: Vec3,
    samples_per_pixel: usize,
    max_depth: usize,
    pixel_samples_scale: f64,
    pixel00_location: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    background_color: Color,
}

impl Camera {
    pub fn very_simple_debug_settings() -> RenderSettings {
        RenderSettings {
            image_width: 10,
            samples_per_pixel: 4,
            max_depth: 10,
        }
    }

    pub fn low_debug_settings() -> RenderSettings {
        RenderSettings {
            image_width: 400,
            samples_per_pixel: 50,
            max_depth: 20,
        }
    }

    pub fn debug_settings() -> RenderSettings {
        RenderSettings {
            image_width: 800,
            samples_per_pixel: 100,
            max_depth: 20,
        }
    }

    pub fn medium_render_settings() -> RenderSettings {
        RenderSettings {
            image_width: 1280,
            samples_per_pixel: 200,
            max_depth: 40,
        }
    }

    pub fn high_render_settings() -> RenderSettings {
        RenderSettings {
            image_width: 1920,
            samples_per_pixel: 500,
            max_depth: 50,
        }
    }

    pub fn very_high_render_settings() -> RenderSettings {
        RenderSettings {
            image_width: 2560,
            samples_per_pixel: 1000,
            max_depth: 50,
        }
    }

    pub fn ultra_high_render_settings() -> RenderSettings {
        RenderSettings {
            image_width: 2560,
            samples_per_pixel: 10000,
            max_depth: 50,
        }
    }

    pub fn four_k_render_settings() -> RenderSettings {
        RenderSettings {
            image_width: 3840,
            samples_per_pixel: 10000,
            max_depth: 50,
        }
    }

    pub fn new(camera_settings: &CameraSettings, render_settings: &RenderSettings) -> Self {
        // Set the camer's image_height to an int no lower than 1
        let image_height =
            (render_settings.image_width as f64 / camera_settings.aspect_ratio) as usize;
        let image_height = std::cmp::max(image_height, 1);

        // Set viewport dimensions
        let theta = camera_settings.fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * camera_settings.focus_dist;
        let viewport_width =
            viewport_height * (render_settings.image_width as f64 / image_height as f64);

        let w = glm::normalize(camera_settings.position - camera_settings.target_position);
        let u = glm::normalize(glm::cross(camera_settings.up_direction, w));
        let v = glm::cross(w, u);

        // Create vectors to line the top and left borders
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Set the distance between the pixel centers in each direction
        let pixel_delta_u = viewport_u / render_settings.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Get the upper left corner in viewport space
        let viewport_upper_left = camera_settings.position
            - w * camera_settings.focus_dist
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);

        // Set the top left pixel location
        let pixel00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius =
            camera_settings.focus_dist * (camera_settings.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width: render_settings.image_width,
            image_height,
            position: camera_settings.position,
            pixel_samples_scale: 1.0 / render_settings.samples_per_pixel as f64,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: render_settings.samples_per_pixel,
            max_depth: render_settings.max_depth,
            defocus_angle: camera_settings.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background_color: camera_settings.background_color,
        }
    }

    fn get_ray(&self, x: f64, y: f64) -> Ray {
        let mut rng = rand::thread_rng();

        let offset = vector::random_unit_square_vec();
        let pixel_sample = self.pixel00_location
            + (self.pixel_delta_u * (x + offset.x))
            + (self.pixel_delta_v * (y + offset.y));

        let ray_origin = match self.defocus_angle <= 0.0 {
            true => self.position,
            false => self.defocus_disk_sample(),
        };

        let ray_time: f64 = rng.gen();

        Ray::new(ray_origin, pixel_sample - ray_origin, ray_time)
    }

    pub fn render(&self, hittable: &dyn HittableObject) {
        eprintln!("Rendering scene...");

        let now = time::Instant::now();

        let num_pixels: usize = self.image_height * self.image_width;

        let mut progress = MappingBar::with_range(0, num_pixels).timed();
        progress.set_len(20);

        let progress = Mutex::new(progress);

        let update_progress = || {
            let mut progress = progress.lock().unwrap();
            progress.add(1_usize);

            if progress.has_progressed_significantly() {
                progress.remember_significant_progress();
                eprintln!("{}", progress);
            }
        };

        let pixels: Vec<Pixel> = (0..num_pixels)
            .into_par_iter()
            .map(|i| {
                let color = self.render_pixel(hittable, i);
                update_progress();
                color
            })
            .collect();

        let mut elapsed = now.elapsed().as_secs();
        let hours = elapsed / 3600;
        elapsed %= 3600;

        let minutes = elapsed / 60;
        let seconds = elapsed % 60;

        eprintln!(
            "Scene renderd in {} hours, {} minutes, {} seconds",
            hours, minutes, seconds
        );

        // Print the PPM header
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        // Print the PPM data
        for pixel in pixels {
            println!("{} {} {}", pixel.x, pixel.y, pixel.z);
        }
    }

    pub fn render_pixel(&self, hittable: &dyn HittableObject, i: usize) -> Pixel {
        let row_index = (i / self.image_width) as f64;
        let column_index = (i % self.image_width) as f64;

        let mut color = (0..self.samples_per_pixel)
            .map(|_| self.get_ray(column_index, row_index))
            .map(|r| self.ray_color(&r, hittable, self.max_depth))
            .reduce(|acc, a| acc + a)
            .unwrap()
            * self.pixel_samples_scale;

        // Use sqrt for gamma correction
        color = vector::sqrt_vec(&color);
        color = vector::clamp_vec3(&color, 0.0..0.999) * 256.0;

        vector::color_to_pixel(&color)
    }

    pub fn ray_color(&self, ray: &Ray, hittable: &dyn HittableObject, depth: usize) -> Color {
        if depth == 0 {
            return vector::zero_vec3();
        }

        let mut record = HitRecord::new(ray);

        let range = 0.001..f64::INFINITY;

        if !hittable.hit(ray, &range, &mut record) {
            return self.background_color;
            // Sky background
            // let unit_direction = glm::normalize(r.direction());
            // let a = (unit_direction.y + 1.0) * 0.5;
            // vector::one_vec3() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
        }

        let emission = record.mat.emitted(&record.uv, &record.point);

        let mut scattered = Ray::new(vector::zero_vec3(), vector::zero_vec3(), ray.time());
        let mut attenuation = vector::zero_vec3();
        let mat = record.mat.clone();

        if !mat.scatter(&mut record, &mut attenuation, &mut scattered) {
            return emission;
        }
        let scatter = self.ray_color(&scattered, hittable, depth - 1) * attenuation;

        emission + scatter
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = vector::random_unit_disk_vec();
        self.position + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }
}
