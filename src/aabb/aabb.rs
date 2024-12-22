use std::{error::Error, fmt::Display};

use crate::{interval::Interval, rays::Ray, vec3::Point3};
#[derive(Default, Clone, Copy, Debug, PartialOrd, PartialEq)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

#[derive(Debug)]
pub enum AABBErrorKind {
    WrongAxis(usize),
}

impl Error for AABBErrorKind {}

impl Display for AABBErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AABBErrorKind::WrongAxis(num) => {
                write!(
                    f,
                    "Error, the axis given doesn't exists. Expected 0 trough 2, got {}",
                    num
                )
            }
        }
    }
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        AABB { x, y, z }
    }
    pub fn from_points(a: Point3, b: Point3) -> Self {
        AABB {
            x: if a.x() <= b.x() {
                Interval::new(a.x(), b.x())
            } else {
                Interval::new(b.x(), a.x())
            },
            y: if a.y() <= b.y() {
                Interval::new(a.y(), b.y())
            } else {
                Interval::new(b.y(), a.y())
            },
            z: if a.z() <= b.z() {
                Interval::new(a.z(), b.z())
            } else {
                Interval::new(b.z(), a.z())
            },
        }
    }
    pub fn from_bbox(a: &AABB, b: &AABB) -> Self {
        let x = if a.x <= b.x {
            Interval::from_intervals(a.x, b.x)
        } else {
            Interval::from_intervals(b.x, a.x)
        };
        let y = Interval::from_intervals(a.y, b.y);
        let z = Interval::from_intervals(a.z, b.z);
        AABB { x, y, z }
    }
    pub fn axis_interval(&self, axis: usize) -> Result<Interval, AABBErrorKind> {
        match axis {
            0 => Ok(self.x),
            1 => Ok(self.y),
            2 => {
                Ok(self.z)
            }
            _ => Err(AABBErrorKind::WrongAxis(axis)),
        }
    }

    pub fn hit(&self, ray: &Ray, time_interval: &Interval) -> bool {
        let mut t_min = time_interval.min();
        let mut t_max = time_interval.max();
        for i in 0..3 {
            let axis = self.axis_interval(i).unwrap();
            let inv_d = 1.0 / ray.direction().as_array()[i];
            let mut t0 = (axis.min() - ray.origin().as_array()[i]) * inv_d;
            let mut t1 = (axis.max() - ray.origin().as_array()[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
    pub fn min(&self) -> Point3 {
        Point3::new(self.x.min(), self.y.min(), self.z.min())
    }
    pub fn max(&self) -> Point3 {
        Point3::new(self.x.max(), self.y.max(), self.z.max())
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point3::new(
        box0.x.min().min(box1.x.min()),
        box0.y.min().min(box1.y.min()),
        box0.z.min().min(box1.z.min()),
    );
    let big = Point3::new(
        box0.x.max().max(box1.x.max()),
        box0.y.max().max(box1.y.max()),
        box0.z.max().max(box1.z.max()),
    );
    AABB::from_points(small, big)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::Ray;
    use crate::vec3::{Point3, Vec3};

    #[test]
    fn test_aabb_hit() {
        let aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        let ray = Ray::new(Point3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), 0.0);
        let time_interval = Interval::new(0.0, 1.0);
        assert_eq!(aabb.hit(&ray, &time_interval), true);
    }
    #[test]
    fn test_aabb_hit_false() {
        let aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        let ray = Ray::new(Point3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, -1.0), 0.0);
        let time_interval = Interval::new(0.0, 1.0);
        assert_eq!(aabb.hit(&ray, &time_interval), false);
    }
    #[test]
    fn test_aabb_surrounding_box() {
        let aabb1 = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        let aabb2 = AABB::new(
            Interval::new(1.0, 2.0),
            Interval::new(1.0, 2.0),
            Interval::new(1.0, 2.0),
        );
        let surrounding = surrounding_box(&aabb1, &aabb2);
        assert_eq!(surrounding.min(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(surrounding.max(), Point3::new(2.0, 2.0, 2.0));
    }
}
