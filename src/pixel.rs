//! Main library for the ray tracer

use glm;

pub fn write_color(color: &glm::DVec3) {
    let color = glm::dvec3(
        color.x.clamp(0.0, 0.999),
        color.y.clamp(0.0, 0.999),
        color.z.clamp(0.0, 0.999),
    );

    let color = color * 256.0;
    let color = glm::ivec3(color.x as i32, color.y as i32, color.z as i32);

    println!("{} {} {}", color.x, color.y, color.z);
}
