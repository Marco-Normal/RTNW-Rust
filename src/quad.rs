use crate::aabb::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval, UNITY_INTERVAL};
use crate::material::Material;
use crate::rays::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Point3, v: Point3, material: Arc<dyn Material>) -> Self {
        let mut bbox = surrounding_box(
            &AABB::from_points(q, q + u + v),
            &AABB::from_points(q + u, q + v),
        );
        bbox.pad_to_minimum(0.0001);

        let n: Vec3 = u.cross_product(&v);
        let normal = n.normalize();
        let d = normal.dot_product(&q);
        let w = n / n.dot_product(&n);

        Quad {
            q,
            u,
            v,
            material,
            bbox,
            normal,
            d,
            w,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, time_interval: &Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot_product(&ray.direction());
        if f64::abs(denominator) < 1e-8 {
            return None;
        }
        let t = (self.d - self.normal.dot_product(&ray.origin())) / denominator;
        if !time_interval.contains(t) {
            return None;
        }
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self
            .w
            .dot_product(&planar_hitpt_vector.cross_product(&self.v));
        let beta = self
            .w
            .dot_product(&self.u.cross_product(&planar_hitpt_vector));
        if !UNITY_INTERVAL.contains(alpha) || !UNITY_INTERVAL.contains(beta) {
            return None;
        }
        let mut rec: HitRecord = Default::default();
        rec.set_t(t);
        rec.set_face_normal(ray, self.normal);
        rec.set_colision_point(ray.at(t));
        rec.set_material(self.material.clone());
        rec.set_u(alpha);
        rec.set_v(beta);
        Some(rec)
    }
    fn bounding_box(&self, _time_interval: &Interval) -> Option<AABB> {
        Some(self.bbox)
    }
}
