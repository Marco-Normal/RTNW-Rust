#![warn(clippy::pedantic)]
use raytracing::{
    bvh::bvh::BVH,
    camera::Camera,
    cmd::cmd_args,
    color::Color,
    common::{random_double, random_double_range},
    cube::Cube,
    hittable::{Hittable, HittableList},
    interval::Interval,
    material::{self, Dielectric, DiffuseLight, Isotropic, Lambertian, Metal},
    medium::ConstantMedium,
    quad::Quad,
    rotation::{AxisRotation, Rotation},
    sphere::Sphere,
    textures::{CheckerPattern, ConstantTexture, ImageTexture, NoiseTexture},
    translate::Translate,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

fn random_scene() -> (Box<dyn Hittable>, Camera) {
    let mut camera: Camera = Default::default();
    camera.set_aspect_ratio(16. / 9.);
    camera.set_width(600);
    camera.set_sample_per_pixel(20);
    camera.set_max_depth(10);
    camera.set_vertical_fov(80.0);
    camera.set_lookfrom(Point3::new(0.0, 0.0, 9.0));
    camera.set_lookat(Point3::new(0.0, 0.0, 0.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    camera.set_focus_distance(10.0);
    camera.set_background_color(Color::new(0.7, 0.8, 1.0));
    let mut world: HittableList = Default::default();
    let checker = Lambertian::new(CheckerPattern::new(
        0.32,
        ConstantTexture::new(Color::new(0.2, 0.3, 0.1)),
        ConstantTexture::new(Color::new(0.9, 0.9, 0.9)),
    ));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        None,
        1000.0,
        Arc::new(checker),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                f64::from(a) + 0.9 * random_double(),
                0.2,
                f64::from(b) + 0.9 * random_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material: Arc<dyn material::Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    material = Arc::new(Lambertian::new(ConstantTexture::new(albedo)));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Box::new(Sphere::new(center, Some(center2), 0.2, material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, None, 0.2, material)));
                } else {
                    // glass
                    material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, None, 0.2, material)));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        None,
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new(ConstantTexture::new(Color::new(
        0.4, 0.2, 0.1,
    ))));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        None,
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        None,
        1.0,
        material3,
    )));
    (
        Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0))),
        camera,
    )
    // Box::new(world)
}

fn checkered_spheres() -> (Box<dyn Hittable>, Camera) {
    let mut camera: Camera = Default::default();
    camera.set_aspect_ratio(16. / 9.);
    camera.set_width(600);
    camera.set_sample_per_pixel(20);
    camera.set_max_depth(10);
    camera.set_vertical_fov(80.0);
    camera.set_lookfrom(Point3::new(0.0, 0.0, 9.0));
    camera.set_lookat(Point3::new(0.0, 0.0, 0.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    camera.set_focus_distance(10.0);
    camera.set_background_color(Color::new(0.7, 0.8, 1.0));

    let mut world: HittableList = Default::default();
    let checker = Lambertian::new(CheckerPattern::new(
        0.32,
        ConstantTexture::from_points(0.2, 0.1, 0.3),
        ConstantTexture::from_points(0.9, 0.9, 0.9),
    ));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        None,
        10.0,
        Arc::new(checker),
    )));

    let checker = Lambertian::new(CheckerPattern::new(
        0.32,
        ConstantTexture::from_points(0.2, 0.1, 0.3),
        ConstantTexture::from_points(0.9, 0.9, 0.9),
    ));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        None,
        10.0,
        Arc::new(checker),
    )));
    (
        Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0))),
        camera,
    )
}

fn perlin_spheres() -> (Box<dyn Hittable>, Camera) {
    let mut camera: Camera = Default::default();
    camera.set_aspect_ratio(16. / 9.);
    camera.set_width(600);
    camera.set_sample_per_pixel(20);
    camera.set_max_depth(10);
    camera.set_vertical_fov(80.0);
    camera.set_lookfrom(Point3::new(0.0, 0.0, 9.0));
    camera.set_lookat(Point3::new(0.0, 0.0, 0.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    camera.set_focus_distance(10.0);
    camera.set_background_color(Color::new(0.7, 0.8, 1.0));
    let mut world: HittableList = Default::default();
    let perlin_texture = NoiseTexture::new(256, 4.0);
    let perlin_sphere = Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        None,
        2.0,
        Arc::new(Lambertian::new(perlin_texture)),
    );

    let perlin_texture = NoiseTexture::new(256, 4.0);
    let ground = Sphere::new(
        Vec3::new(0.0, -1200.0, 0.0),
        None,
        1200.0,
        Arc::new(Lambertian::new(perlin_texture)),
    );
    world.add(Box::new(perlin_sphere));
    world.add(Box::new(ground));
    (
        Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0))),
        camera,
    )
}

fn earth() -> (Box<dyn Hittable>, Camera) {
    let mut camera: Camera = Default::default();
    camera.set_aspect_ratio(16. / 9.);
    camera.set_width(600);
    camera.set_sample_per_pixel(20);
    camera.set_max_depth(10);
    camera.set_vertical_fov(80.0);
    camera.set_lookfrom(Point3::new(0.0, 0.0, 9.0));
    camera.set_lookat(Point3::new(0.0, 0.0, 0.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    camera.set_focus_distance(10.0);
    camera.set_background_color(Color::new(0.7, 0.8, 1.0));
    let mut world: HittableList = Default::default();
    let earth_texture = ImageTexture::from("earthmap.png".to_string());
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let earth = Box::new(Sphere::new(Point3::default(), None, 2.0, earth_surface));
    world.add(earth);
    (
        Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0))),
        camera,
    )
}

fn boxes() -> (Box<dyn Hittable>, Camera) {
    let mut camera: Camera = Default::default();
    camera.set_aspect_ratio(16. / 9.);
    camera.set_width(600);
    camera.set_sample_per_pixel(20);
    camera.set_max_depth(10);
    camera.set_vertical_fov(80.0);
    camera.set_lookfrom(Point3::new(0.0, 0.0, 9.0));
    camera.set_lookat(Point3::new(0.0, 0.0, 0.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    camera.set_focus_distance(10.0);
    camera.set_background_color(Color::new(0.7, 0.8, 1.0));
    let mut world: HittableList = Default::default();
    let left_red = Arc::new(Lambertian::new(ConstantTexture::new(Color::new(
        1.0, 0.2, 0.2,
    ))));
    let back_green = Arc::new(Lambertian::new(ConstantTexture::new(Color::new(
        0.2, 1.0, 0.2,
    ))));
    let right_blue = Arc::new(Lambertian::new(ConstantTexture::new(Color::new(
        0.2, 0.2, 1.0,
    ))));
    let upper_orange = Arc::new(Lambertian::new(ConstantTexture::new(Color::new(
        1.0, 0.5, 0.0,
    ))));
    let lower_teal = Arc::new(Lambertian::new(ConstantTexture::new(Color::new(
        0.2, 0.8, 0.8,
    ))));
    world.add(Box::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));
    (
        Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0))),
        camera,
    )
}

fn simple_light() -> (Box<dyn Hittable>, Camera) {
    let mut camera: Camera = Default::default();
    camera.set_aspect_ratio(16. / 9.);
    camera.set_width(600);
    camera.set_sample_per_pixel(20);
    camera.set_max_depth(10);
    camera.set_vertical_fov(20.0);
    camera.set_lookfrom(Point3::new(26.0, 3.0, 6.0));
    camera.set_lookat(Point3::new(0.0, 2.0, 0.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    camera.set_focus_distance(10.0);

    let mut world: HittableList = Default::default();
    let perlin_texture = NoiseTexture::new(256, 4.0);

    let ground = Lambertian::new(CheckerPattern::new(
        0.32,
        ConstantTexture::from_points(0.2, 0.1, 0.3),
        ConstantTexture::from_points(0.9, 0.9, 0.9),
    ));
    let diff_light = DiffuseLight::new(ConstantTexture::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        None,
        2.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        None,
        1000.0,
        Arc::new(ground),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Arc::new(diff_light),
    )));

    let diff_light = DiffuseLight::new(ConstantTexture::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        None,
        2.0,
        Arc::new(diff_light),
    )));
    (
        Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0))),
        camera,
    )
}

fn cornell_box() -> (Box<dyn Hittable>, Camera) {
    let mut camera: Camera = Default::default();
    camera.set_width(400);
    camera.set_sample_per_pixel(50);
    camera.set_aspect_ratio(1.0);
    camera.set_max_depth(50);
    camera.set_vertical_fov(40.0);
    camera.set_lookfrom(Point3::new(278.0, 278.0, -800.0));
    camera.set_lookat(Point3::new(278.0, 278.0, 0.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    camera.set_focus_distance(10.0);
    camera.set_background_color(Color::new(0.0, 0.0, 0.0));
    let mut world: HittableList = Default::default();
    let red = Lambertian::new(ConstantTexture::new(Color::new(0.65, 0.05, 0.05)));
    let green = Lambertian::new(ConstantTexture::new(Color::new(0.12, 0.45, 0.15)));
    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let light = DiffuseLight::new(ConstantTexture::new(Color::new(15.0, 15.0, 15.0)));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::new(red),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::new(green),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::new(white),
    )));
    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Arc::new(white),
    )));

    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Arc::new(white),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-80.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -130.0),
        Arc::new(light),
    )));
    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let box1 = Translate::new(
        Rotation::new(
            Cube::new(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 333.0, 165.0),
                Arc::new(white),
            ),
            AxisRotation::Yaxis,
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );

    world.add(Box::new(box1));

    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let box2 = Translate::new(
        Rotation::new(
            Cube::new(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 165.0, 165.0),
                Arc::new(white),
            ),
            AxisRotation::Yaxis,
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );
    world.add(Box::new(box2));
    (
        Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0))),
        camera,
    )
}

fn cornell_box_smoke() -> (Box<dyn Hittable>, Camera) {
    let mut camera: Camera = Default::default();
    camera.set_width(600);
    camera.set_aspect_ratio(1.0);
    camera.set_sample_per_pixel(200);
    camera.set_max_depth(50);
    camera.set_vertical_fov(40.0);
    camera.set_lookfrom(Point3::new(278.0, 278.0, -800.0));
    camera.set_lookat(Point3::new(278.0, 278.0, 0.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    camera.set_focus_distance(10.0);
    camera.set_background_color(Color::new(0.0, 0.0, 0.0));
    let mut world: HittableList = Default::default();
    let red = Lambertian::new(ConstantTexture::new(Color::new(0.65, 0.05, 0.05)));
    let green = Lambertian::new(ConstantTexture::new(Color::new(0.12, 0.45, 0.15)));
    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let light = DiffuseLight::new(ConstantTexture::new(Color::new(7.0, 7.0, 7.0)));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::new(red),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::new(green),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::new(white),
    )));
    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Arc::new(white),
    )));

    let white = Isotropic::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Arc::new(white),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        Arc::new(light),
    )));
    let white = Isotropic::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let box1 = Translate::new(
        Rotation::new(
            Cube::new(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 333.0, 165.0),
                Arc::new(white),
            ),
            AxisRotation::Yaxis,
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );

    world.add(Box::new(ConstantMedium::new(
        box1,
        0.01,
        ConstantTexture::new(Color::new(1.0, 1.0, 1.0)),
    )));

    let white = Isotropic::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let box2 = Translate::new(
        Rotation::new(
            Cube::new(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 165.0, 165.0),
                Arc::new(white),
            ),
            AxisRotation::Yaxis,
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );
    world.add(Box::new(ConstantMedium::new(
        box2,
        0.01,
        ConstantTexture::new(Color::default()),
    )));
    (
        Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0))),
        camera,
    )
}

fn final_scene(
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,
) -> (Box<dyn Hittable>, Camera) {
    let mut world: HittableList = Default::default();
    let mut boxes1: HittableList = Default::default();
    let mut camera: Camera = Default::default();
    camera.set_aspect_ratio(16.0 / 9.0);
    camera.set_width(image_width);
    camera.set_sample_per_pixel(samples_per_pixel);
    camera.set_max_depth(max_depth);
    camera.set_background_color(Color::default());
    camera.set_vertical_fov(40.0);
    camera.set_lookfrom(Vec3::new(478.0, 278.0, -600.0));
    camera.set_lookat(Vec3::new(278.0, 278.0, 278.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    let ground: Arc<Lambertian<ConstantTexture>> =
        Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53).into()));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Box::new(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    world.add(Box::new(BVH::new(boxes1.objects, &Interval::new(0.0, 1.0))));
    let light: DiffuseLight<ConstantTexture> = DiffuseLight::new(Color::new(7.0, 7.0, 7.0).into());
    world.add(Box::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        Arc::new(light),
    )));
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material: Arc<Lambertian<ConstantTexture>> =
        Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1).into()));
    world.add(Box::new(Sphere::new(
        center1,
        Some(center2),
        50.0,
        sphere_material,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        None,
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        None,
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));
    let boundary = Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        None,
        70.0,
        Arc::new(Dielectric::new(1.5)),
    );
    world.add(Box::new(ConstantMedium::new(
        boundary,
        0.0001,
        ConstantTexture::new(Color::new(1.0, 1.0, 1.0)),
    )));
    let pertext = Arc::new(Lambertian::new(NoiseTexture::new(256, 0.2)));
    world.add(Box::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        None,
        80.0,
        pertext,
    )));
    let mut boxes2: HittableList = Default::default();
    let white: Arc<Lambertian<ConstantTexture>> =
        Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73).into()));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Box::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            None,
            10.0,
            white.clone(),
        )))
    }
    world.add(Box::new(Translate::new(
        Rotation::new(
            BVH::new(boxes2.objects, &Interval::new(0.0, 1.0)),
            AxisRotation::Yaxis,
            15.0,
        ),
        Vec3::new(-100.0, 270.0, 395.0),
    )));
    let emat = Arc::new(Lambertian::new(ImageTexture::from(
        "earthmap.png".to_string(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        None,
        100.0,
        emat,
    )));
    (
        Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0))),
        camera,
    )
}

fn main() {
    // World
    let (world, mut camera) = final_scene(1080, 5000 / 2, 50);
    let filename = cmd_args().unwrap();
    camera.render(&world, filename);
}
