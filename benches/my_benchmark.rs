use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracing::camera::Camera;
use raytracing::hittable::{Hittable, HittableList};
use raytracing::interval::Interval;
use raytracing::material::{Dielectric, Lambertian, Material, Metal};
use raytracing::rays::Ray;
use raytracing::sphere::Sphere;
use raytracing::vec3::{reflect, reflectance, refract, Vec3};

pub fn bench_vec3_add(c: &mut Criterion) {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    c.bench_function("Vec3 add", |b| {
        b.iter(|| {
            black_box(v1 + v2);
        })
    });
}

pub fn bench_vec3_sub(c: &mut Criterion) {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    c.bench_function("Vec3 sub", |b| {
        b.iter(|| {
            black_box(v1 - v2);
        })
    });
}

pub fn bench_vec3_mul(c: &mut Criterion) {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    c.bench_function("Vec3 mul", |b| {
        b.iter(|| {
            black_box(v1 * v2);
        })
    });
}

pub fn bench_vec3_div(c: &mut Criterion) {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = 0.2;
    c.bench_function("Vec3 div", |b| {
        b.iter(|| {
            black_box(v1 / v2);
        })
    });
}

pub fn bench_vec3_dot(c: &mut Criterion) {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    c.bench_function("Vec3 dot", |b| {
        b.iter(|| {
            black_box(v1.dot_product(&v2));
        })
    });
}

pub fn bench_vec3_cross(c: &mut Criterion) {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    c.bench_function("Vec3 cross", |b| {
        b.iter(|| {
            black_box(v1.cross_product(&v2));
        })
    });
}

pub fn bench_vec3_length(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("Vec3 length", |b| {
        b.iter(|| {
            black_box(v.magnitude());
        })
    });
}

pub fn bench_vec3_length_squared(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("Vec3 length_squared", |b| {
        b.iter(|| {
            black_box(v.square_magnitude());
        })
    });
}

pub fn bench_vec3_unit_vector(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("Vec3 unit_vector", |b| {
        b.iter(|| {
            black_box(v.normalize());
        })
    });
}

pub fn bench_reflection(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let n = Vec3::new(4.0, 5.0, 6.0);
    c.bench_function("Reflection", |b| {
        b.iter(|| {
            black_box(reflect(&v, &n));
        })
    });
}

pub fn bench_refraction(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let n = Vec3::new(4.0, 5.0, 6.0);
    c.bench_function("Refraction", |b| {
        b.iter(|| {
            black_box(refract(&v, &n, 0.5));
        })
    });
}

pub fn bench_ray_at(c: &mut Criterion) {
    let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
    c.bench_function("Ray at", |b| {
        b.iter(|| {
            black_box(r.at(7.0));
        })
    });
}

pub fn bench_colision_sphere(c: &mut Criterion) {
    let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
    let s = Sphere::new(
        Vec3::new(1.0, 2.0, 3.0),
        4.0,
        Arc::new(Lambertian::new(Vec3::new(1.0, 1.0, 1.0))),
    );
    c.bench_function("Colision sphere", |b| {
        b.iter(|| {
            black_box(s.hit(&r, &Interval::new(0., 100.)));
        })
    });
}

pub fn bench_colision_list(c: &mut Criterion) {
    let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
    let s1 = Sphere::new(
        Vec3::new(1.0, 2.0, 3.0),
        4.0,
        Arc::new(Lambertian::new(Vec3::new(1.0, 1.0, 1.0))),
    );
    let s2 = Sphere::new(
        Vec3::new(1.0, 2.0, 3.0),
        4.0,
        Arc::new(Lambertian::new(Vec3::new(1.0, 1.0, 1.0))),
    );
    let mut list = HittableList::new();
    list.add(Box::new(s1));
    list.add(Box::new(s2));
    c.bench_function("Colision list", |b| {
        b.iter(|| {
            black_box(list.hit(&r, &Interval::new(0., 100.)));
        })
    });
}

pub fn bench_metal_reflectance(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let n = Vec3::new(4.0, 5.0, 6.0);
    let cos_theta = f64::min(-v.normalize().dot_product(&n), 1.0);
    let r0 = (1.0 - 0.5) / (1.0 + 0.5);
    c.bench_function("Metal reflectance", |b| {
        b.iter(|| {
            black_box(reflectance(cos_theta, r0));
        })
    });
}

fn simple_scene() -> HittableList {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    )));
    world
}

pub fn bench_camera(c: &mut Criterion) {
    let lookfrom = Vec3::new(1.0, 2.0, 3.0);
    let lookat = Vec3::new(4.0, 5.0, 6.0);
    let vup = Vec3::new(7.0, 8.0, 9.0);
    let vfov = 90.0;
    let aspect_ratio = 16.0 / 9.0;
    let aperture = 0.1;
    let focus_dist = 10.0;
    let mut camera = Camera::default();
    camera.set_lookat(lookat);
    camera.set_lookfrom(lookfrom);
    camera.set_vup(vup);
    camera.set_vertical_fov(vfov);
    camera.set_aspect_ratio(aspect_ratio);
    camera.set_defocus_angle(aperture);
    camera.set_focus_distance(focus_dist);
    let world = simple_scene();
    c.bench_function("Camera render", |b| {
        b.iter(|| {
            black_box(camera.render(&world, "test.ppm".to_string()));
        })
    });
}

pub fn fn_bench_all(c: &mut Criterion) {
    bench_vec3_add(c);
    bench_vec3_sub(c);
    bench_vec3_mul(c);
    bench_vec3_div(c);
    bench_vec3_dot(c);
    bench_vec3_cross(c);
    bench_vec3_length(c);
    bench_vec3_length_squared(c);
    bench_vec3_unit_vector(c);
    bench_reflection(c);
    bench_refraction(c);
    bench_ray_at(c);
    bench_colision_sphere(c);
    bench_colision_list(c);
    bench_metal_reflectance(c);
    bench_camera(c);
}

criterion_group!(benches, fn_bench_all);

criterion_main!(benches);
