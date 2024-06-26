//! A module for managing textures

use crate::vector;
use crate::vector::{Color, Vec2, Vec3};
use image::{imageops, io::Reader as ImageReader, ImageBuffer, Rgb};
use noise::{NoiseFn, Perlin, Seedable, Turbulence};
use std::fmt;
use std::sync::Arc;

pub trait Texture: fmt::Debug + Send + Sync {
    fn sample(&self, uv: &Vec2, point: &Vec3) -> Color;
}

#[derive(Debug, Copy, Clone)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn from(color: Color) -> Self {
        Self { color }
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self::from(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn sample(&self, _uv: &Vec2, _point: &Vec3) -> Color {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct CheckerBoard {
    inverse_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerBoard {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inverse_scale: scale.recip(),
            even: even.clone(),
            odd: odd.clone(),
        }
    }

    pub fn from_colors(scale: f64, even_color: Color, odd_color: Color) -> Self {
        let even_texture = Arc::new(SolidColor::from(even_color));
        let odd_texture = Arc::new(SolidColor::from(odd_color));

        Self {
            inverse_scale: scale.recip(),
            even: even_texture,
            odd: odd_texture,
        }
    }
}

impl Texture for CheckerBoard {
    fn sample(&self, uv: &Vec2, point: &Vec3) -> Color {
        let point = *point * self.inverse_scale;
        let x = point.x.floor() as i32;
        let y = point.y.floor() as i32;
        let z = point.z.floor() as i32;

        match (x + y + z) % 2 {
            0 => self.even.sample(uv, &point),
            _ => self.odd.sample(uv, &point),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageTexture {
    image: ImageBuffer<Rgb<f32>, Vec<f32>>,
    // image: DynamicImage,
}

impl ImageTexture {
    pub fn new(file_path: &str) -> Self {
        Self {
            // TODO: should definitely be failing gracefully here
            // instead of using unwrap.
            image: ImageReader::open(file_path)
                .unwrap()
                .decode()
                .unwrap()
                .to_rgb32f(),
        }
    }
}

impl Texture for ImageTexture {
    fn sample(&self, uv: &Vec2, _point: &Vec3) -> Color {
        let (_, height) = self.image.dimensions();

        if height == 0 {
            // Return magenta so that texture errors are obvious
            return Color::new(1.0, 0.0, 1.0);
        }

        // Just clamp instead of repeating, etc.
        let uv = vector::clamp_vec2(uv, 0.0..1.0);

        // The image library uses f32 instead of f64, so we'll need
        // to scale down to sample the texture.
        let u = uv.x as f32;

        // Invert the vertical component because images are 0 on top
        let v = 1.0 - uv.y as f32;

        let pixel_data = match imageops::sample_bilinear(&self.image, u, v) {
            Some(pixel_data) => pixel_data,
            // Return magenta so that missing textures are obvious
            None => Rgb::<f32>([1.0, 0.0, 1.0]),
        };

        Color::new(
            pixel_data[0] as f64,
            pixel_data[1] as f64,
            pixel_data[2] as f64,
        )
    }
}

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: Turbulence<Perlin, Perlin>,
    scale: f64,
    phase: f64,
}

impl NoiseTexture {
    pub fn new(
        seed: u32,
        scale: f64,
        phase: f64,
        roughness: usize,
        frequency: f64,
        power: f64,
    ) -> Self {
        let noise = Turbulence::new(Perlin::default())
            .set_seed(seed)
            .set_power(power)
            .set_roughness(roughness)
            .set_frequency(frequency);

        Self {
            noise,
            scale,
            phase,
        }
    }
}

impl Texture for NoiseTexture {
    fn sample(&self, _uv: &Vec2, point: &Vec3) -> Color {
        let noise = self.scale * point.z + self.noise.get(*point.as_array()) * self.phase;
        Color::new(0.5, 0.5, 0.5) * (1.0 + noise.sin())
    }
}
