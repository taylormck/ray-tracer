//! Main library for the ray tracer

use glm;

pub fn write_color(color: &glm::DVec3) {
    let color = *color * 255.999;
    let color = glm::ivec3(color.x as i32, color.y as i32, color.z as i32);
    println!("{} {} {}", color.x, color.y, color.z);
}
