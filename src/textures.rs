use crate::{
    color::Color, image::texture_map::read_image, interval::Interval, perlin::Perlin, vec3::Point3,
};

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
#[derive(Default)]
pub struct ImageTexture {
    image: Vec<u8>,
    ux: u32,
    uy: u32,
}
#[derive(Clone, Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
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

impl ImageTexture {
    pub fn new(image: Vec<u8>, ux: u32, uy: u32) -> Self {
        ImageTexture { image, ux, uy }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.uy <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        let i = (Interval::new(0.0, 1.0).clamp(u) * self.ux as f64) as usize;
        let j = ((1.0 - Interval::new(0.0, 1.0).clamp(v)) * self.uy as f64) as usize;
        let idx: usize = 3 * i + 3 * self.ux as usize * j;
        let r = self.image[idx] as f64 / 255.0;
        let g = self.image[idx + 1] as f64 / 255.0;
        let b = self.image[idx + 2] as f64 / 255.0;
        Color::new(r, g, b)
    }
}

impl NoiseTexture {
    pub fn new(point_count: usize, scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(point_count),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.noise.turbulence(p, 10)))
    }
}

impl From<Color> for ConstantTexture {
    fn from(value: Color) -> Self {
        Self { albedo: value }
    }
}

impl From<String> for ImageTexture {
    fn from(value: String) -> Self {
        match read_image(value) {
            Ok(texture) => texture,
            Err(_e) => ImageTexture::default(),
        }
    }
}
