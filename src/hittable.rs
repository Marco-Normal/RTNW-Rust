use crate::aabb::aabb::{surrounding_box, AABB};
use crate::interval::Interval;
use crate::material::Material;
use crate::rays::Ray;
use crate::vec3::{Point3, Vec3};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    mat: Option<Arc<dyn Material>>,
    u: f64,
    v: f64,
}
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Debug for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HitRecord")
            .field("p", &self.p)
            .field("normal", &self.normal)
            .field("t", &self.t)
            .field("front_face", &self.front_face)
            .finish()
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn p(&self) -> Point3 {
        self.p
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn u(&self) -> f64 {
        self.u
    }
    pub fn v(&self) -> f64 {
        self.v
    }
    pub fn set_u(&mut self, u: f64) {
        self.u = u
    }

    pub fn set_v(&mut self, v: f64) {
        self.v = v
    }
    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }
    pub fn set_colision_point(&mut self, point: Point3) {
        self.p = point;
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot_product(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
    pub fn set_material(&mut self, material: Arc<dyn Material>) {
        self.mat = Some(material);
    }
    pub fn get_material(&self) -> Option<Arc<dyn Material>> {
        self.mat.clone()
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, new: Box<dyn Hittable>) {
        self.objects.push(new);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, time_interval: &Interval) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = time_interval.max();
        for object in &self.objects {
            if let Some(rec) = object.hit(ray, &Interval::new(time_interval.min(), closest_so_far))
            {
                closest_so_far = rec.t();
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
    fn bounding_box(&self, time_interval: &Interval) -> Option<AABB> {
        match &self.objects.first() {
            Some(first) => match first.bounding_box(&time_interval) {
                Some(bbox) => self.objects.iter().skip(1).try_fold(bbox, |acc, hittable| {
                    match hittable.bounding_box(&time_interval) {
                        Some(bbox) => Some(surrounding_box(&acc, &&bbox)),
                        _ => None,
                    }
                }),
                _ => None,
            },
            _ => None,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, time_interval: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self, time_interval: &Interval) -> Option<AABB>;
}
