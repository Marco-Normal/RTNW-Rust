use crate::common::INFINITY;
use std::ops::{Add, AddAssign};
pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
pub const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};
pub const UNITY_INTERVAL: Interval = Interval { min: 0.0, max: 1.0 };
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Interval {
    min: f64,
    max: f64,
}
impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }
    pub fn from_intervals(a: Interval, b: Interval) -> Self {
        let min = a.min().min(b.min());
        let max = a.max().max(b.max());
        Interval { min, max }
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn set_min(&mut self, min: f64) {
        self.min = min;
    }
    pub fn max(&self) -> f64 {
        self.max
    }
    pub fn set_max(&mut self, max: f64) {
        self.max = max;
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, number: f64) -> bool {
        self.min <= number && number <= self.max
    }
    pub fn surround(&self, number: f64) -> bool {
        self.min < number && number < self.max
    }
    pub fn clamp(&self, number: f64) -> f64 {
        if number < self.min {
            self.min
        } else if number > self.max {
            self.max
        } else {
            number
        }
    }
    pub fn expand_inplace(&mut self, delta: f64) {
        let padding = delta / 2.0;
        self.min -= padding;
        self.max += padding;
    }
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;
    fn add(self, rhs: f64) -> Self {
        Interval {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl AddAssign<f64> for Interval {
    fn add_assign(&mut self, rhs: f64) {
        self.min = rhs + self.min;
        self.max = rhs + self.max;
    }
}
