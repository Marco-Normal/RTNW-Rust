use crate::common::random_double;
use crate::common::INFINITY;
use crate::{
    hittable::{HitRecord, Hittable},
    interval::{Interval, UNIVERSE},
    material::Isotropic,
    rays::Ray,
    textures::Texture,
    vec3::Vec3,
};
use std::f64::consts::E;
use std::sync::Arc;

pub struct ConstantMedium<H: Hittable, T: Texture> {
    boundary: H,
    neg_inv_density: f64,
    phase_function: Arc<Isotropic<T>>,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, density: f64, texture: T) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(texture)),
        }
    }
}

impl<H: Hittable, T: Texture + 'static> Hittable for ConstantMedium<H, T> {
    fn hit(&self, ray: &Ray, time_interval: &Interval) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, &UNIVERSE) {
            if let Some(mut rec2) = self
                .boundary
                .hit(ray, &Interval::new(rec1.t() + 0.0001, INFINITY))
            {
                if rec1.t() < time_interval.min() {
                    rec1.set_t(time_interval.min());
                }
                if rec2.t() > time_interval.max() {
                    rec2.set_t(time_interval.max())
                }
                if rec1.t() >= rec2.t() {
                    return None;
                }
                if rec1.t() < 0.0 {
                    rec1.set_t(0.0)
                }
                let ray_length = ray.direction().square_magnitude();
                let distance_inside_boundary = (rec2.t() - rec1.t()) * ray_length;
                let hit_distance = self.neg_inv_density * random_double().ln();
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let mut rec: HitRecord = Default::default();
                rec.set_t(rec1.t() + hit_distance / ray_length);
                rec.set_colision_point(ray.at(rec.t()));
                rec.set_normal(Vec3::new(1.0, 0.0, 0.0)); // Arbitrary
                rec.set_material(self.phase_function.clone());
                return Some(rec);
            }
            return None;
        }
        return None;
    }
    fn bounding_box(&self, time_interval: &Interval) -> Option<crate::aabb::aabb::AABB> {
        self.boundary.bounding_box(time_interval)
    }
}
