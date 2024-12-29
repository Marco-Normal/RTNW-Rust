use crate::common::{random_double, random_double_range};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
#[derive(Clone, Copy, Debug, PartialEq)]
/// Implementation of a 3D vector
/// # Examples
/// ```
/// use raytracer::vec3::Vec3;
/// let a = Vec3::new(1., 2., 3.);
/// let b = Vec3::new(4., 5., 6.);
/// let c = a.cross_product(&b);
/// assert_eq!(c.x(), -3.);
/// assert_eq!(c.y(), 6.);
/// assert_eq!(c.z(), -3.);
/// ```
/// # Notes
/// All the fields are private, so you need to use the getters and setters to access them
/// # Fields
/// * `x` - The x coordinate of the vector
/// * `y` - The y coordinate of the vector
/// * `z` - The z coordinate of the vector
/// # Methods
/// * `new(x: f64, y: f64, z: f64) -> Vec3` - Creates a new Vec3 with the given coordinates
/// * `unit_vector() -> Vec3` - Creates a new Vec3 with all coordinates set to 1
/// * `x() -> f64` - Returns the x coordinate of the vector
/// * `y() -> f64` - Returns the y coordinate of the vector
/// * `z() -> f64` - Returns the z coordinate of the vector
/// * `set_x(x: f64)` - Sets the x coordinate of the vector
/// * `set_y(y: f64)` - Sets the y coordinate of the vector
/// * `set_z(z: f64)` - Sets the z coordinate of the vector
/// * `square_magnitude() -> f64` - Returns the square of the magnitude of the vector
/// * `magnitude() -> f64` - Returns the magnitude of the vector
/// * `normalize_inplace()` - Normalizes the vector in place
/// * `normalize() -> Vec3` - Returns a new normalized vector
/// * `dot_product(other: &Vec3) -> f64` - Returns the dot product of the vector with another vector
/// * `cross_product_inplace(other: &Vec3)` - Calculates the cross product of the vector with another vector in place
/// * `cross_product(other: &Vec3) -> Vec3` - Calculates the cross product of the vector with another vector
/// * `random() -> Vec3` - Returns a random vector
/// * `random_range(min: f64, max: f64) -> Vec3` - Returns a random vector with coordinates in the given range
/// * `near_zero() -> bool` - Returns true if the vector is near zero
///
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
pub type Point3 = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn unit_vector() -> Self {
        Vec3::new(1., 1., 1.)
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }
    pub fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.square_magnitude())
    }
    pub fn normalize_inplace(&mut self) {
        // Possível bug pode vir daqui
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn normalize(&self) -> Self {
        // Possível bug pode vir daqui
        let magnitude = self.magnitude();
        Vec3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }
    pub fn dot_product(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross_product_inplace(&mut self, other: &Vec3) {
        let new_x = self.y * other.z - self.z * other.y;
        let new_y = self.z * other.x - self.x * other.z;
        let new_z = self.x * other.y - self.y * other.x;
        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }
    pub fn cross_product(&self, other: &Vec3) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn random() -> Self {
        Self::new(random_double(), random_double(), random_double())
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
    pub fn as_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    pub fn axis(&self, axis: usize) -> f64 {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("No axis founded"),
        }
    }
    pub fn set_axis(&mut self, axis: usize, value: f64) {
        match axis {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => panic!("No axis founded"),
        }
    }
}
/// Returns a random vector with coordinates in the range [0, 1)
pub fn random_unit_vector() -> Vec3 {
    loop {
        let point: Vec3 = Vec3::random_range(-1., 1.);
        let magnitude: f64 = point.square_magnitude();
        if 1e-160 < magnitude && magnitude <= 1.0 {
            return point.normalize();
        }
    }
}
/// Returns a random vector with coordinates in the range [-1, 1) Sampled from a cosine distribution
pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere: Vec3 = random_unit_vector();
    if on_unit_sphere.dot_product(&normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
/// Returns a random vector with coordinates in the range [-1, 1) Sampled from a cosine distribution
pub fn random_on_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double_range(-1., 1.),
            random_double_range(-1., 1.),
            0.,
        );
        if p.square_magnitude() < 1. {
            return p;
        }
    }
}
/// Returns the reflection of a vector
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot_product(n) * *n
}
/// Returns the simulation of a refraction
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(uv.dot_product(n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.square_magnitude())) * *n;
    r_out_perp + r_out_parallel
}
/// Returns the reflectance of a material
pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 = r0 * r0;
    r0 + (1. - r0) * f64::powi(1. - cosine, 5)
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x / self, rhs.y / self, rhs.z / self)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_product() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        let c = a.cross_product(&b);
        assert_eq!(c.x(), -3.);
        assert_eq!(c.y(), 6.);
        assert_eq!(c.z(), -3.);
    }
    #[test]
    fn test_array() {
        let a = Vec3::new(1., 2., 3.);
        let b = a.as_array();
        assert_eq!(b[0], 1.);
        assert_eq!(b[1], 2.);
        assert_eq!(b[2], 3.);
    }
}
