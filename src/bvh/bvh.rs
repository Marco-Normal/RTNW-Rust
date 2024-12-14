use crate::{
    aabb::aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    rays::Ray,
};

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

pub struct BVH {
    root: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut hittable: Vec<Box<dyn Hittable>>, time_interval: &Interval) -> Self {
        fn box_compare(
            time_interval: &Interval,
            axis: usize,
        ) -> impl for<'a, 'b> FnMut(
            &'a Box<(dyn Hittable + 'static)>,
            &'b Box<(dyn Hittable + 'static)>,
        ) -> std::cmp::Ordering
               + use<'_> {
            move |a, b| {
                let a_bbox = a.bounding_box(time_interval);
                let b_bbox = b.bounding_box(time_interval);
                if let (Some(a_bbox), Some(b_bbox)) = (a_bbox, b_bbox) {
                    let ac = a_bbox.min().as_array()[axis] + a_bbox.max().as_array()[axis];
                    let bc = b_bbox.min().as_array()[axis] + b_bbox.max().as_array()[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!("No bounding box")
                }
            }
        }
        fn axis_range(
            hittable: &Vec<Box<dyn Hittable>>,
            time_interval: &Interval,
            axis: usize,
        ) -> f64 {
            let (min, max) = hittable
                .iter()
                .fold((f64::MIN, f64::MAX), |(bmin, bmax), hit| {
                    if let Some(aabb) = hit.bounding_box(time_interval) {
                        (
                            bmin.min(aabb.min().as_array()[axis]),
                            bmax.max(aabb.max().as_array()[axis]),
                        )
                    } else {
                        (bmin, bmax)
                    }
                });
            max - min
        }
        let mut axis_ranges: Vec<(usize, f64)> = (0..3)
            .map(|a| (a, axis_range(&hittable, &time_interval, a)))
            .collect();
        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let axis = axis_ranges[0].0;
        hittable.sort_unstable_by(box_compare(&time_interval, axis));
        let len = hittable.len();
        match len {
            0 => {
                panic!("No elements in scene");
            }
            1 => {
                let leaf = hittable.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time_interval) {
                    return BVH {
                        root: BVHNode::Leaf(leaf),
                        bbox,
                    };
                } else {
                    panic!("No bounding box");
                }
            }
            _ => {
                let right = BVH::new(hittable.drain(len / 2..).collect(), &time_interval);
                let left = BVH::new(hittable, &time_interval);
                let bbox = surrounding_box(&left.bbox, &right.bbox);
                return BVH {
                    root: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                };
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, time_interval: &Interval) -> Option<HitRecord> {
        if self.bbox.hit(&ray, &time_interval) {
            match &self.root {
                BVHNode::Branch { left, right } => {
                    let left_bbox = left.hit(&ray, &time_interval);
                    let right_bbox = if left_bbox.is_some() {
                        right.hit(
                            &ray,
                            &Interval::new(time_interval.min(), left_bbox.as_ref().unwrap().t()),
                        )
                    } else {
                        right.hit(&ray, &time_interval)
                    };
                    match (left_bbox, right_bbox) {
                        (h, None) | (None, h) => h,
                        (Some(left_hit), Some(right_hit)) => {
                            if left_hit.t() < right_hit.t() {
                                Some(left_hit)
                            } else {
                                Some(right_hit)
                            }
                        }
                    }
                }
                BVHNode::Leaf(leaf) => {
                    return leaf.hit(&ray, &time_interval);
                }
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, _time_interval: &Interval) -> Option<AABB> {
        Some(self.bbox)
    }
}

#[cfg(test)]
mod tests {
    use std::{f64::INFINITY, sync::Arc};

    use crate::sphere::Sphere;

    use super::*;

    #[test]
    fn new() {
        let sphere1 = Sphere::new(
            crate::vec3::Vec3::new(0.0, 0.0, 0.0),
            None,
            1.0,
            Arc::new(crate::material::Lambertian::new(crate::vec3::Vec3::new(
                0.0, 0.0, 0.0,
            ))),
        );
        let sphere2 = Sphere::new(
            crate::vec3::Vec3::new(0.0, 0.0, 0.0),
            None,
            1.0,
            Arc::new(crate::material::Lambertian::new(crate::vec3::Vec3::new(
                0.0, 0.0, 0.0,
            ))),
        );
        let bvh = BVH::new(
            vec![
                Box::new(sphere1) as Box<dyn Hittable>,
                Box::new(sphere2) as Box<dyn Hittable>,
            ],
            &Interval::new(0.0, 1.0),
        );
        assert!(bvh.bounding_box(&Interval::new(0.0, 1.0)).is_some());
    }

    #[test]
    fn test_collision() {
        let sphere1 = Sphere::new(
            crate::vec3::Vec3::new(0.0, 0.0, 1.0),
            None,
            1.0,
            Arc::new(crate::material::Lambertian::new(crate::vec3::Vec3::new(
                0.0, 0.0, 0.0,
            ))),
        );
        let sphere2 = Sphere::new(
            crate::vec3::Vec3::new(0.0, 0.0, 2.0),
            None,
            1.0,
            Arc::new(crate::material::Lambertian::new(crate::vec3::Vec3::new(
                0.0, 0.0, 0.0,
            ))),
        );
        let bvh = BVH::new(
            vec![Box::new(sphere1) as Box<dyn Hittable>],
            &Interval::new(0.0, 1.0),
        );
        let ray = Ray::new(
            crate::vec3::Vec3::new(0.0, 0.0, 0.0),
            crate::vec3::Vec3::new(0.0, 0.0, 1.0),
            0.5,
        );
        let hit = bvh.hit(&ray, &Interval::new(0.0, INFINITY));
        assert!(hit.is_some());
    }
    #[test]
    fn test_bbox() {
        let sphere1 = Sphere::new(
            crate::vec3::Vec3::new(0.0, 0.0, 2.0),
            None,
            1.0,
            Arc::new(crate::material::Lambertian::new(crate::vec3::Vec3::new(
                0.0, 0.0, 0.0,
            ))),
        );
        let sphere2 = Sphere::new(
            crate::vec3::Vec3::new(0.0, 0.0, 0.0),
            None,
            1.0,
            Arc::new(crate::material::Lambertian::new(crate::vec3::Vec3::new(
                0.0, 0.0, 0.0,
            ))),
        );
        let bvh = BVH::new(
            vec![
                Box::new(sphere1) as Box<dyn Hittable>,
                Box::new(sphere2) as Box<dyn Hittable>,
            ],
            &Interval::new(0.0, 1.0),
        );
        let bbox = bvh.bounding_box(&Interval::new(0.0, 1.0)).unwrap();
        assert_eq!(bbox.min(), crate::vec3::Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(bbox.max(), crate::vec3::Vec3::new(1.0, 1.0, 3.0));
    }
}
