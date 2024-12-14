use raytracing::{
    bvh::bvh::BVH,
    camera::Camera,
    cmd::cmd_args,
    color::Color,
    common::{random_double, random_double_range},
    hittable::{Hittable, HittableList},
    interval::Interval,
    material::{self, Dielectric, Lambertian, Metal},
    sphere::Sphere,
    textures::{CheckerPattern, ConstantTexture, Texture},
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

fn random_scene() -> Box<dyn Hittable> {
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
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
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
    Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0)))
    // Box::new(world)
}

fn checkered_spheres() -> Box<dyn Hittable> {
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
    Box::new(BVH::new(world.objects, &Interval::new(0.0, 1.0)))
}

fn main() {
    // World
    let world = random_scene();
    let mut camera: Camera = Default::default();
    let filename = cmd_args().unwrap();
    camera.set_aspect_ratio(16. / 9.);
    camera.set_width(1200);
    camera.set_sample_per_pixel(250);
    camera.set_max_depth(50);
    camera.set_vertical_fov(20.0);
    camera.set_lookfrom(Point3::new(13.0, 2.0, 3.0));
    camera.set_lookat(Point3::new(0.0, 0.0, 0.0));
    camera.set_vup(Vec3::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.6);
    camera.set_focus_distance(10.);
    camera.render(&world, filename);
}
