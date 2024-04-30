//! A simple raytracer in Rust
//! I'm building this both to practice Rust and to
//! brush up on graphics programming in general.

use rand::Rng;
use ray_tracer_rust::{
    bvh::BVHNode,
    camera::{Camera, CameraSettings},
    material::{refraction_indices, Dielectric, Lambertian, Material, Metal},
    quad::Quad,
    scene::Scene,
    sphere::Sphere,
    texture::{CheckerBoard, ImageTexture, NoiseTexture},
    vector,
    vector::{Color, Vec3},
};

use clap::Parser;
use std::sync::Arc;

fn render_checkered_spheres_scene(settings: &CameraSettings) {
    let mut scene = Scene::new();

    let checker_texture = Arc::new(CheckerBoard::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let checker_texture = Arc::new(Lambertian::from_texture(checker_texture));

    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        vector::zero_vec3(),
        10.0,
        checker_texture.clone(),
    )));

    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        vector::zero_vec3(),
        10.0,
        checker_texture.clone(),
    )));

    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        vector::zero_vec3(),
        vector::up_vec3(),
        20.0,
        settings,
    );

    camera.render(&scene);
}

fn render_earth_scene(settings: &CameraSettings) {
    let mut scene = Scene::new();

    // let earth_texture_neat = Arc::new(ImageTexture::new("./images/earth-map-neat.jpg"));
    // let earth_surface_neat = Arc::new(Lambertian::from_texture(earth_texture_neat));

    let earth_texture_realistic = Arc::new(ImageTexture::new("./images/earth-realistic.jpg"));
    let earth_surface_realistic = Arc::new(Lambertian::from_texture(earth_texture_realistic));

    let scifi_planet_texture_realistic = Arc::new(ImageTexture::new("./images/scifi-planet.jpg"));
    let scifi_planet_surface_realistic =
        Arc::new(Lambertian::from_texture(scifi_planet_texture_realistic));

    scene.add(Arc::new(Sphere::new(
        Vec3::new(-2.3, 0.0, 0.0),
        vector::zero_vec3(),
        2.0,
        earth_surface_realistic,
    )));

    scene.add(Arc::new(Sphere::new(
        Vec3::new(2.3, 0.0, 0.0),
        vector::zero_vec3(),
        2.0,
        scifi_planet_surface_realistic,
    )));

    // scene.add(Arc::new(Sphere::new(
    //     Vec3::new(2.0, 0.0, 0.0),
    //     vector::zero_vec3(),
    //     1.5,
    //     earth_surface_neat,
    // )));

    let camera = Camera::new(
        Vec3::new(-4.0, -2.0, 9.0),
        vector::zero_vec3(),
        vector::up_vec3(),
        35.0,
        settings,
    );

    camera.render(&scene);
}

fn render_bouncing_balls_scene(settings: &CameraSettings) {
    let mut rng = rand::thread_rng();

    let mut scene = Scene::new();

    // Ground
    let checker_texture = Arc::new(CheckerBoard::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    // let material_ground = Arc::new(Lambertian::from_color_components(0.5, 0.5, 0.5));
    let material_ground = Arc::new(Lambertian::from_texture(checker_texture));

    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        vector::zero_vec3(),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();

            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if glm::ext::sqlength(center - glm::dvec3(4.0, 0.2, 0.0)) > 0.81 {
                let mut velocity = vector::zero_vec3();

                let sphere_material: Arc<dyn Material> = match choose_mat {
                    choose_mat if choose_mat < 0.8 => {
                        let albedo = vector::random_vec3(0.0..1.0) * vector::random_vec3(0.0..1.0);
                        velocity.y = rng.gen_range(0.0..0.5);
                        Arc::new(Lambertian::from_color(albedo))
                    }
                    choose_mat if choose_mat < 0.9 => {
                        let albedo = vector::random_vec3(0.5..1.0);
                        let fuzz: f64 = rng.gen_range(0.0..0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    }
                    _ => {
                        let albedo = glm::dvec3(1.0, 1.0, 1.0);
                        let opacity = 0.0;
                        Arc::new(Dielectric::new(albedo, opacity, refraction_indices::GLASS))
                    }
                };

                scene.add(Arc::new(Sphere::new(
                    center,
                    velocity,
                    0.2,
                    sphere_material,
                )));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(
        glm::dvec3(1.0, 1.0, 1.0),
        1.5,
        refraction_indices::GLASS,
    ));

    scene.add(Arc::new(Sphere::new(
        glm::dvec3(0.0, 1.0, 0.0),
        vector::zero_vec3(),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::from_color(glm::dvec3(0.4, 0.2, 0.1)));

    scene.add(Arc::new(Sphere::new(
        glm::dvec3(-4.0, 1.0, 0.0),
        vector::zero_vec3(),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(glm::dvec3(0.4, 0.6, 0.5), 0.0));

    scene.add(Arc::new(Sphere::new(
        glm::dvec3(4.0, 1.0, 0.0),
        vector::zero_vec3(),
        1.0,
        material3,
    )));

    let tree = BVHNode::from(scene.objects());

    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        vector::zero_vec3(),
        vector::up_vec3(),
        20.0,
        settings,
    );

    camera.render(&tree);
}

fn render_perlin_spheres_scene(settings: &CameraSettings) {
    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_material = Arc::new(Lambertian::from_texture(perlin_texture));

    let mut scene = Scene::new();

    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        vector::zero_vec3(),
        1000.0,
        perlin_material.clone(),
    )));

    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        vector::zero_vec3(),
        2.0,
        perlin_material.clone(),
    )));

    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        vector::zero_vec3(),
        vector::up_vec3(),
        20.0,
        settings,
    );

    camera.render(&scene);
}

fn render_quads_scene(settings: &CameraSettings) {
    let left_red = Arc::new(Lambertian::from_color_components(1.0, 0.2, 0.2));
    let back_green = Arc::new(Lambertian::from_color_components(0.2, 1.0, 0.2));
    let right_blue = Arc::new(Lambertian::from_color_components(0.2, 0.2, 1.0));
    let upper_orange = Arc::new(Lambertian::from_color_components(1.0, 0.5, 0.0));
    let lower_teal = Arc::new(Lambertian::from_color_components(0.2, 0.8, 0.8));

    let mut scene = Scene::new();

    scene.add(Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut settings = settings.clone();
    settings.aspect_ratio = 1.0;

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 9.0),
        vector::zero_vec3(),
        vector::up_vec3(),
        80.0,
        &settings,
    );

    camera.render(&scene);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    scene: u8,

    #[arg(short, long, default_value_t = 1)]
    camera: u8,
}

fn main() {
    let args = Args::parse();

    let settings = match args.camera {
        0 => Camera::very_simple_debug_settings(),
        1 => Camera::debug_settings(),
        2 => Camera::high_render_settings(),
        3 => Camera::very_high_render_settings(),
        4 => Camera::probably_too_high_render_settings(),
        _ => {
            eprintln!("Invalid camera settings");
            std::process::exit(1);
        }
    };

    match args.scene {
        0 => render_bouncing_balls_scene(&settings),
        1 => render_checkered_spheres_scene(&settings),
        2 => render_earth_scene(&settings),
        3 => render_perlin_spheres_scene(&settings),
        4 => render_quads_scene(&settings),
        _ => {
            eprintln!("Invalid scene id");
            std::process::exit(1);
        }
    }
}
