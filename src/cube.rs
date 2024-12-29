use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable, HittableList},
    material::Material,
    quad::Quad,
    vec3::{Point3, Vec3},
};

pub struct Cube {
    sides: HittableList,
}

impl Cube {
    pub fn new<T: Material + 'static>(a: Point3, b: Point3, material: Arc<T>) -> Self {
        let mut sides: HittableList = Default::default();
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            material.clone(),
        )));

        sides.add(Box::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            material.clone(),
        )));

        sides.add(Box::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            material.clone(),
        )));

        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            material.clone(),
        )));

        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            material.clone(),
        )));

        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            material,
        )));
        Cube { sides }
    }
}

impl Hittable for Cube {
    fn hit(
        &self,
        ray: &crate::rays::Ray,
        time_interval: &crate::interval::Interval,
    ) -> Option<HitRecord> {
        self.sides.hit(ray, time_interval)
    }
    fn bounding_box(
        &self,
        time_interval: &crate::interval::Interval,
    ) -> Option<crate::aabb::aabb::AABB> {
        self.sides.bounding_box(time_interval)
    }
}
