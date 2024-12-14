use std::sync::Arc;

use crate::aabb::aabb::{surrounding_box, AABB};
use crate::common::PI;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::rays::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Arc<dyn Material>,
    bbox: AABB,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, time_interval: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time());
        let trajeto = ray.origin() - current_center;
        let a = ray.direction().dot_product(&ray.direction());
        let h = trajeto.dot_product(&ray.direction());
        let c = trajeto.square_magnitude() - self.radius * self.radius;
        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = f64::sqrt(discriminant);
        let root = (-h - sqrt_d) / a;
        if !time_interval.surround(root) {
            return None;
        }
        let mut rec = HitRecord::new();
        rec.set_t(root);
        rec.set_colision_point(ray.at(root));
        rec.set_normal((rec.p() - current_center) / self.radius);
        rec.set_material(self.material.clone());
        let (u, v) = self.get_sphere_uv(&rec.normal());
        rec.set_u(u);
        rec.set_v(v);

        Some(rec)
    }
    fn bounding_box(&self, _time_interval: &Interval) -> Option<AABB> {
        Some(self.bbox)
    }
}

impl Sphere {
    pub fn new(
        first_center: Point3,
        second_center: Option<Point3>,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        if let Some(next_center) = second_center {
            let center = Ray::new(first_center, next_center - first_center, 0.0);
            let bbox1 = AABB::from_points(first_center - rvec, first_center + rvec);
            let bbox2 = AABB::from_points(next_center - rvec, next_center + rvec);

            Sphere {
                center,
                radius,
                material,
                bbox: surrounding_box(&bbox1, &bbox2),
            }
        } else {
            let bbox = AABB::from_points(first_center - rvec, first_center + rvec);
            Sphere {
                center: Ray::new(first_center, Default::default(), 0.0),
                radius,
                material,
                bbox,
            }
        }
    }
    pub fn center(&self) -> Point3 {
        self.center.origin()
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn bbox(&self) -> AABB {
        self.bbox
    }
    pub fn get_sphere_uv(&self, p: &Point3) -> (f64, f64) {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

#[cfg(test)]
mod tests {
    use crate::{material::Lambertian, textures::ConstantTexture};

    use super::*;

    #[test]
    fn check_bbox() {
        let sphere = Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            None,
            1.0,
            Arc::new(Lambertian::new(ConstantTexture::from_points(0.1, 0.2, 0.3))),
        );
        let bbox = sphere.bbox();
        assert_eq!(bbox.min(), Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(bbox.max(), Vec3::new(1.0, 1.0, 1.0));
    }
    #[test]
    fn check_moving_bbox() {
        let sphere = Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            Some(Point3::new(1.0, 0.0, 0.0)),
            1.0,
            Arc::new(Lambertian::new(ConstantTexture::from_points(0.1, 0.2, 0.3))),
        );
        let bbox = sphere.bbox();
        assert_eq!(bbox.min(), Vec3::new(-1.0, -1.0, -1.0), "min");
        assert_eq!(bbox.max(), Vec3::new(2.0, 1.0, 1.0), "max");
    }
    #[test]
    fn check_hit() {
        let sphere = Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            None,
            1.0,
            Arc::new(Lambertian::new(ConstantTexture::from_points(0.1, 0.2, 0.3))),
        );
        let ray = Ray::new(Point3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0), 0.0);
        let interval = Interval::new(0.0, f64::INFINITY);
        let hit = sphere.hit(&ray, &interval);
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert_eq!(hit.t(), 1.0);
        assert_eq!(hit.p(), Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(hit.normal(), Vec3::new(0.0, 0.0, -1.0));
    }
}
