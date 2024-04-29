//! A module for managing textures

use crate::vector::{Color, Vec2, Vec3};
use std::fmt;
use std::sync::Arc;

pub trait Texture: fmt::Debug + Send + Sync {
    fn sample(self: &Self, uv: &Vec2, point: &Vec3) -> Color;
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
    fn sample(self: &Self, _uv: &Vec2, _point: &Vec3) -> Color {
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
    fn sample(self: &Self, uv: &Vec2, point: &Vec3) -> Color {
        let point = *point * self.inverse_scale;
        let x = point.x.floor() as i32;
        let y = point.y.floor() as i32;
        let z = point.z.floor() as i32;

        match (x + y + z) as i32 % 2 {
            0 => self.even.sample(uv, &point),
            _ => self.odd.sample(uv, &point),
        }
    }
}
