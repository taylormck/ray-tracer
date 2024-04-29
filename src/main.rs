//! A simple raytracer in Rust
//! I'm building this both to practice Rust and to
//! brush up on graphics programming in general.

use rand::Rng;
use ray_tracer_rust::bvh::BVHNode;
use ray_tracer_rust::camera::Camera;
use ray_tracer_rust::material::{refraction_indices, Dielectric, Lambertian, Material, Metal};
use ray_tracer_rust::scene::Scene;
use ray_tracer_rust::sphere::Sphere;
use ray_tracer_rust::texture::{CheckerBoard, ImageTexture};
use ray_tracer_rust::vector;
use ray_tracer_rust::vector::{Color, Vec3};
use std::sync::Arc;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 80;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 20;
const FOV: f64 = 35.0;
const DEFOCUS_ANGLE: f64 = 0.0;
const FOCUS_DIST: f64 = 10.0;

fn render_checkered_spheres_scene() {
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

    const CAMERA_POSITION: glm::DVec3 = glm::DVec3 {
        x: 0.0,
        y: 0.0,
        z: 100.0,
    };

    const CAMERA_TARGET: glm::DVec3 = glm::DVec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    const CAMERA_UP: glm::DVec3 = glm::DVec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let camera = Camera::new(
        CAMERA_POSITION,
        CAMERA_TARGET,
        CAMERA_UP,
        ASPECT_RATIO,
        IMAGE_WIDTH,
        FOV,
        DEFOCUS_ANGLE,
        FOCUS_DIST,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
    );

    camera.render(&scene);
}

fn render_earth_scene() {
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
    //
    const CAMERA_POSITION: glm::DVec3 = glm::DVec3 {
        x: -4.0,
        y: -2.0,
        z: 9.0,
    };

    const CAMERA_TARGET: glm::DVec3 = glm::DVec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    const CAMERA_UP: glm::DVec3 = glm::DVec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let camera = Camera::new(
        CAMERA_POSITION,
        CAMERA_TARGET,
        CAMERA_UP,
        ASPECT_RATIO,
        IMAGE_WIDTH,
        FOV,
        DEFOCUS_ANGLE,
        FOCUS_DIST,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
    );

    camera.render(&scene);
}

fn render_bouncing_balls_scene() {
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

    const CAMERA_POSITION: glm::DVec3 = glm::DVec3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };

    const CAMERA_TARGET: glm::DVec3 = glm::DVec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    const CAMERA_UP: glm::DVec3 = glm::DVec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let camera = Camera::new(
        CAMERA_POSITION,
        CAMERA_TARGET,
        CAMERA_UP,
        ASPECT_RATIO,
        IMAGE_WIDTH,
        FOV,
        DEFOCUS_ANGLE,
        FOCUS_DIST,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
    );

    camera.render(&tree);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Please provide a scene id as an argument.");
        std::process::exit(1);
    }

    let scene_id = &args[1];

    let scene_id = match scene_id.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("You must pass a non-negative integer as the only parameter");
            std::process::exit(1);
        }
    };

    match scene_id {
        0 => render_bouncing_balls_scene(),
        1 => render_checkered_spheres_scene(),
        2 => render_earth_scene(),
        _ => {
            eprintln!("Invalid scene id");
            std::process::exit(1);
        }
    }
}
