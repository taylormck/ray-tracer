//! A simple raytracer in Rust
//! I'm building this both to practice Rust and to
//! brush up on graphics programming in general.

use rand::Rng;
use ray_tracer_rust::{
    bvh::BVHNode,
    camera::{Camera, CameraSettings, RenderSettings},
    constant_medium::ConstantMedium,
    hittable::{HittableList, RotateY, Translate},
    material::{refraction_indices, Dielectric, DiffuseLight, Lambertian, Material, Metal},
    quad::Quad,
    sphere::Sphere,
    texture::{CheckerBoard, ImageTexture, NoiseTexture},
    vector,
    vector::{Color, Vec3},
};

use clap::Parser;
use std::sync::Arc;

fn render_checkered_spheres_scene(render_settings: &RenderSettings) {
    let mut scene = HittableList::new();

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
        &CameraSettings {
            position: Vec3::new(13.0, 2.0, 3.0),
            target_position: vector::zero_vec3(),
            up_direction: vector::up_vec3(),
            fov: 20.0,
            aspect_ratio: 16.0 / 9.0,
            defocus_angle: 0.6,
            focus_dist: 10.0,
            background_color: Color::new(0.7, 0.8, 1.0),
        },
        render_settings,
    );

    camera.render(&scene);
}

fn render_earth_scene(render_settings: &RenderSettings) {
    let mut scene = HittableList::new();

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
        &CameraSettings {
            position: Vec3::new(-4.0, -2.0, 9.0),
            target_position: vector::zero_vec3(),
            up_direction: vector::up_vec3(),
            fov: 35.0,
            aspect_ratio: 16.0 / 9.0,
            defocus_angle: 0.6,
            focus_dist: 10.0,
            background_color: Color::new(0.7, 0.8, 1.0),
        },
        render_settings,
    );

    camera.render(&scene);
}

fn render_bouncing_balls_scene(render_settings: &RenderSettings) {
    let mut rng = rand::thread_rng();

    let mut scene = HittableList::new();

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
                    _ => Arc::new(Dielectric::new(refraction_indices::GLASS)),
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

    let material1 = Arc::new(Dielectric::new(refraction_indices::GLASS));

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
        &CameraSettings {
            position: Vec3::new(13.0, 2.0, 3.0),
            target_position: vector::zero_vec3(),
            up_direction: vector::up_vec3(),
            fov: 20.0,
            aspect_ratio: 16.0 / 9.0,
            defocus_angle: 0.6,
            focus_dist: 10.0,
            background_color: Color::new(0.7, 0.8, 1.0),
        },
        render_settings,
    );

    camera.render(&tree);
}

fn render_perlin_spheres_scene(render_settings: &RenderSettings) {
    let perlin_texture = Arc::new(NoiseTexture::new(0, 4.0, 10.0, 6, 1.0, 1.0));
    let perlin_material = Arc::new(Lambertian::from_texture(perlin_texture));

    let mut scene = HittableList::new();

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
        &CameraSettings {
            position: Vec3::new(13.0, 2.0, 3.0),
            target_position: Vec3::new(0.0, 0.0, 0.0),
            up_direction: vector::up_vec3(),
            fov: 20.0,
            aspect_ratio: 16.0 / 9.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            background_color: Color::new(0.7, 0.8, 1.0),
        },
        render_settings,
    );

    camera.render(&scene);
}

fn render_simple_light_scene(render_settings: &RenderSettings) {
    let perlin_texture = Arc::new(NoiseTexture::new(0, 4.0, 10.0, 6, 1.0, 1.0));
    let perlin_material = Arc::new(Lambertian::from_texture(perlin_texture));

    let mut scene = HittableList::new();

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

    let diff_light_material = Arc::new(DiffuseLight::from_color_components(4.0, 4.0, 4.0));

    scene.add(Arc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diff_light_material.clone(),
    )));

    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        vector::zero_vec3(),
        2.0,
        diff_light_material.clone(),
    )));

    let camera = Camera::new(
        &CameraSettings {
            position: Vec3::new(26.0, 3.0, 6.0),
            target_position: Vec3::new(0.0, 2.0, 0.0),
            up_direction: vector::up_vec3(),
            fov: 20.0,
            aspect_ratio: 16.0 / 9.0,
            defocus_angle: 0.6,
            focus_dist: 10.0,
            background_color: Color::new(0.0, 0.0, 0.0),
        },
        render_settings,
    );

    camera.render(&scene);
}

fn render_quads_scene(render_settings: &RenderSettings) {
    let left_red = Arc::new(Lambertian::from_color_components(1.0, 0.2, 0.2));
    let back_green = Arc::new(Lambertian::from_color_components(0.2, 1.0, 0.2));
    let right_blue = Arc::new(Lambertian::from_color_components(0.2, 0.2, 1.0));
    let upper_orange = Arc::new(Lambertian::from_color_components(1.0, 0.5, 0.0));
    let lower_teal = Arc::new(Lambertian::from_color_components(0.2, 0.8, 0.8));

    let mut scene = HittableList::new();

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

    let camera = Camera::new(
        &CameraSettings {
            position: Vec3::new(0.0, 0.0, 9.0),
            target_position: vector::zero_vec3(),
            up_direction: vector::up_vec3(),
            fov: 80.0,
            aspect_ratio: 1.0,
            defocus_angle: 0.6,
            focus_dist: 10.0,
            background_color: Color::new(0.7, 0.8, 1.0),
        },
        render_settings,
    );

    camera.render(&scene);
}

fn render_cornell_box_scene(render_settings: &RenderSettings) {
    let red = Arc::new(Lambertian::from_color_components(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::from_color_components(0.73, 0.73, 0.73));
    let green = Arc::new(Lambertian::from_color_components(0.12, 0.45, 0.15));
    let light = Arc::new(DiffuseLight::from_color_components(15.0, 15.0, 15.0));

    let mut scene = HittableList::new();

    scene.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1 = Quad::box_from_opposite_corners(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    );

    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    scene.add(box1);

    let mut box2 = Quad::box_from_opposite_corners(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    );

    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    scene.add(box2);

    let camera = Camera::new(
        &CameraSettings {
            position: Vec3::new(278.0, 278.0, -800.0),
            target_position: Vec3::new(278.0, 278.0, 0.0),
            up_direction: vector::up_vec3(),
            fov: 40.0,
            aspect_ratio: 1.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            background_color: Color::new(0.0, 0.0, 0.0),
        },
        render_settings,
    );

    camera.render(&scene);
}

fn render_cornell_smoke_box_scene(render_settings: &RenderSettings) {
    let red = Arc::new(Lambertian::from_color_components(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::from_color_components(0.73, 0.73, 0.73));
    let green = Arc::new(Lambertian::from_color_components(0.12, 0.45, 0.15));
    let light = Arc::new(DiffuseLight::from_color_components(7.0, 7.0, 7.0));

    let mut scene = HittableList::new();

    scene.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    scene.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1 = Quad::box_from_opposite_corners(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    );

    box1 = Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    ));

    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    scene.add(box1);

    let mut box2 = Quad::box_from_opposite_corners(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    );

    box2 = Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    ));

    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    scene.add(box2);

    let camera = Camera::new(
        &CameraSettings {
            position: Vec3::new(278.0, 278.0, -800.0),
            target_position: Vec3::new(278.0, 278.0, 0.0),
            up_direction: vector::up_vec3(),
            fov: 40.0,
            aspect_ratio: 1.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            background_color: Color::new(0.0, 0.0, 0.0),
        },
        render_settings,
    );

    camera.render(&scene);
}

fn render_final_scene(render_settings: &RenderSettings) {
    let mut rng = rand::thread_rng();

    let mut scene = HittableList::new();

    // Floor boxes
    let mut boxes_1 = HittableList::new();
    let ground = Arc::new(Lambertian::from_color_components(0.48, 0.83, 0.53));
    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let a = Vec3::new(-1000.0 + i as f64 * w, 0.0, -1000.0 + j as f64 * w);
            let b = Vec3::new(a.x + w, rng.gen_range(1.0..101.0), a.z + w);

            boxes_1.add(Quad::box_from_opposite_corners(a, b, ground.clone()));
        }
    }

    scene.add(Arc::new(BVHNode::from(boxes_1.objects())));

    let light = Arc::new(DiffuseLight::from_color_components(7.0, 7.0, 7.0));
    scene.add(Arc::new(Quad::new(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    // Moving orange ball in the
    scene.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 400.0, 200.0),
        Vec3::new(30.0, 0.0, 0.0),
        50.0,
        Arc::new(Lambertian::from_color_components(0.73, 0.3, 0.1)),
    )));

    // Glass ball in the bottom center
    scene.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        vector::zero_vec3(),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    // Metal ball in the bottom right
    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        vector::zero_vec3(),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // Blue glass ball in the bottom left
    let boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        vector::zero_vec3(),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    // Filling for the ball above
    scene.add(boundary.clone());
    scene.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    // Outer shell of the scene
    let boundary = Arc::new(Sphere::new(
        vector::zero_vec3(),
        vector::zero_vec3(),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    // // Thin mist around the entire scene
    scene.add(boundary.clone());
    scene.add(Arc::new(ConstantMedium::from_color(
        boundary,
        1e-4,
        Color::new(1.0, 1.0, 1.0),
    )));

    // Earth ball
    let earth_texture_realistic = Arc::new(ImageTexture::new("./images/earth-realistic.jpg"));
    let earth_surface_realistic = Arc::new(Lambertian::from_texture(earth_texture_realistic));
    scene.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        vector::zero_vec3(),
        100.0,
        earth_surface_realistic,
    )));

    // Perlin noise ball
    let perlin_texture = Arc::new(NoiseTexture::new(0, 0.2, 0.1, 6, 1.0, 1.0));
    let perlin_material = Arc::new(Lambertian::from_texture(perlin_texture));

    scene.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        vector::zero_vec3(),
        80.0,
        perlin_material,
    )));

    // Box full of balls in the top right corner
    let mut boxes_2 = HittableList::new();
    let white_material = Arc::new(Lambertian::from_color_components(0.73, 0.73, 0.73));
    let ns = 1000;

    for _ in 0..ns {
        boxes_2.add(Arc::new(Sphere::new(
            vector::random_vec3(0.0..165.0),
            vector::zero_vec3(),
            10.0,
            white_material.clone(),
        )));
    }

    scene.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(boxes_2), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let camera = Camera::new(
        &CameraSettings {
            position: Vec3::new(478.0, 278.0, -600.0),
            target_position: Vec3::new(278.0, 278.0, 0.0),
            up_direction: vector::up_vec3(),
            fov: 40.0,
            aspect_ratio: 1.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            background_color: Color::new(0.0, 0.0, 0.0),
        },
        render_settings,
    );

    camera.render(&scene);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    scene: u8,

    #[arg(short, long, default_value_t = 1)]
    quality: u8,
}

fn main() {
    let args = Args::parse();

    let render_settings = match args.quality {
        0 => Camera::very_simple_debug_settings(),
        1 => Camera::low_debug_settings(),
        2 => Camera::debug_settings(),
        3 => Camera::medium_render_settings(),
        4 => Camera::high_render_settings(),
        5 => Camera::very_high_render_settings(),
        6 => Camera::ultra_high_render_settings(),
        7 => Camera::four_k_render_settings(),
        _ => {
            eprintln!("Invalid camera settings");
            std::process::exit(1);
        }
    };

    match args.scene {
        0 => render_bouncing_balls_scene(&render_settings),
        1 => render_checkered_spheres_scene(&render_settings),
        2 => render_earth_scene(&render_settings),
        3 => render_perlin_spheres_scene(&render_settings),
        4 => render_simple_light_scene(&render_settings),
        5 => render_quads_scene(&render_settings),
        6 => render_cornell_box_scene(&render_settings),
        7 => render_cornell_smoke_box_scene(&render_settings),
        8 => render_final_scene(&render_settings),
        _ => {
            eprintln!("Invalid scene id");
            std::process::exit(1);
        }
    }
}
