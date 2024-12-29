use crate::{
    aabb::aabb::AABB,
    common::{degree_to_radians, INFINITY},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    rays::Ray,
    vec3::Point3,
};

pub struct Rotation<H: Hittable> {
    rotation: AxisRotation,
    object: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

pub enum AxisRotation {
    Xaxis,
    Yaxis,
    Zaxis,
}
fn axis_index(axis: &AxisRotation) -> (usize, usize, usize) {
    match axis {
        AxisRotation::Xaxis => (0, 1, 2),
        AxisRotation::Yaxis => (1, 2, 0),
        AxisRotation::Zaxis => (2, 0, 1),
    }
}
impl<H: Hittable> Rotation<H> {
    pub fn new(object: H, axis_rotation: AxisRotation, angle: f64) -> Self {
        let (r_axis, a_axis, b_axis) = axis_index(&axis_rotation);
        let bbox = object.bounding_box(&Interval::new(0.0, 1.0));
        let radians = degree_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        if let Some(bbox) = bbox {
            let mut max_point = Point3::new(INFINITY, INFINITY, INFINITY);
            let mut min_point = Point3::new(-INFINITY, -INFINITY, -INFINITY);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let r = k as f64 * bbox.max().axis(r_axis)
                            + (1.0 - k as f64) * bbox.min().axis(r_axis);
                        let a = i as f64 * bbox.max().axis(a_axis)
                            + (1.0 - i as f64) * bbox.min().axis(a_axis);
                        let b = j as f64 * bbox.max().axis(b_axis)
                            + (1.0 - j as f64) * bbox.min().axis(b_axis);
                        let new_a = cos_theta * a - sin_theta * b;
                        let new_b = sin_theta * a + cos_theta * b;
                        if new_a < min_point.axis(a_axis) {
                            min_point.set_axis(a_axis, new_a);
                        }

                        if new_b < min_point.axis(b_axis) {
                            min_point.set_axis(b_axis, new_b);
                        }

                        if r < min_point.axis(r_axis) {
                            min_point.set_axis(r_axis, r);
                        }

                        if new_a > max_point.axis(a_axis) {
                            max_point.set_axis(a_axis, new_a);
                        }

                        if new_b > max_point.axis(b_axis) {
                            max_point.set_axis(b_axis, new_b);
                        }

                        if r > max_point.axis(r_axis) {
                            max_point.set_axis(r_axis, r);
                        }
                    }
                }
            }
            let bbox = AABB::from_points(min_point, max_point);
            Rotation {
                rotation: axis_rotation,
                object,
                sin_theta,
                cos_theta,
                bbox: Some(bbox),
            }
        } else {
            panic!("No bbox found")
        }
    }
}

impl<H: Hittable> Hittable for Rotation<H> {
    fn hit(&self, ray: &Ray, time_interval: &Interval) -> Option<HitRecord> {
        let (_, a_axis, b_axis) = axis_index(&self.rotation);
        let mut origin = ray.origin();
        let mut direction = ray.direction();
        origin.set_axis(
            a_axis,
            self.cos_theta * ray.origin().axis(a_axis) + self.sin_theta * ray.origin().axis(b_axis),
        );
        origin.set_axis(
            b_axis,
            self.cos_theta * ray.origin().axis(b_axis) - self.sin_theta * ray.origin().axis(a_axis),
        );
        direction.set_axis(
            a_axis,
            self.cos_theta * ray.direction().axis(a_axis)
                + self.sin_theta * ray.direction().axis(b_axis),
        );
        direction.set_axis(
            b_axis,
            self.cos_theta * ray.direction().axis(b_axis)
                - self.sin_theta * ray.direction().axis(a_axis),
        );
        let rotated_ray = Ray::new(origin, direction, ray.time());
        if let Some(mut rec) = self.object.hit(&rotated_ray, &time_interval) {
            let mut p = rec.p();
            let mut normal = rec.normal();
            p.set_axis(
                a_axis,
                self.cos_theta * rec.p().axis(a_axis) - self.sin_theta * rec.p().axis(b_axis),
            );
            p.set_axis(
                b_axis,
                self.cos_theta * rec.p().axis(b_axis) + self.sin_theta * rec.p().axis(a_axis),
            );
            normal.set_axis(
                a_axis,
                self.cos_theta * rec.normal().axis(a_axis)
                    - self.sin_theta * rec.normal().axis(b_axis),
            );
            normal.set_axis(
                b_axis,
                self.cos_theta * rec.normal().axis(b_axis)
                    + self.sin_theta * rec.normal().axis(a_axis),
            );
            rec.set_normal(normal);
            rec.set_colision_point(p);
            return Some(rec);
        }
        None
    }
    fn bounding_box(&self, _time_interval: &Interval) -> Option<AABB> {
        self.bbox
    }
}
