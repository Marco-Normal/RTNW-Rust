use crate::vec3::*;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            tm: time,
        }
    }
    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
}
