use crate::{color::Color, vec3::Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
#[derive(Default)]
pub struct ConstantTexture {
    albedo: Color,
}

#[derive(Default)]
pub struct CheckerPattern<T: Texture, U: Texture> {
    scale: f64,
    inv_scale: f64,
    even: T,
    odd: U,
}

impl ConstantTexture {
    pub fn new(color: Color) -> Self {
        ConstantTexture { albedo: color }
    }
    pub fn from_points(r: f64, g: f64, b: f64) -> Self {
        ConstantTexture {
            albedo: Color::new(r, g, b),
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}
impl<T: Texture, U: Texture> CheckerPattern<T, U> {
    pub fn new(scale: f64, even: T, odd: U) -> Self {
        CheckerPattern {
            scale,
            inv_scale: 1. / scale,
            even,
            odd,
        }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerPattern<T, U> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_int = (p.x() * self.inv_scale).floor() as i32;
        let y_int = (p.y() * self.inv_scale).floor() as i32;
        let z_int = (p.z() * self.inv_scale).floor() as i32;
        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

impl From<Color> for ConstantTexture {
    fn from(value: Color) -> Self {
        Self { albedo: value }
    }
}
