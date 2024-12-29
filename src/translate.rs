use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    rays::Ray,
    vec3::Vec3,
};
pub struct Translate<H: Hittable> {
    object: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(object: H, offset: Vec3) -> Self {
        Translate { object, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, ray: &Ray, time_interval: &Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        if let Some(mut rec) = self.object.hit(&offset_ray, time_interval) {
            let new_p = rec.p() + self.offset;
            rec.set_colision_point(new_p);
            return Some(rec);
        }
        None
    }
    fn bounding_box(&self, time_interval: &Interval) -> Option<crate::aabb::aabb::AABB> {
        self.object.bounding_box(time_interval).map(|mut b| {
            b.set_max(self.offset);
            b
        })
    }
}
