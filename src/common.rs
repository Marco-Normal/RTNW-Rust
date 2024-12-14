use rand::Rng;
use std::f64::consts;

pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = consts::PI;

pub fn degree_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
