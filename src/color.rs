use crate::interval::Interval;
use crate::vec3::Vec3;
use image::ImageBuffer;
use std::fs::File;
use std::io::prelude::*;
pub type Color = Vec3;
impl Color {
    pub fn get_r(&self) -> f64 {
        self.x()
    }

    pub fn get_g(&self) -> f64 {
        self.y()
    }
    pub fn get_b(&self) -> f64 {
        self.z()
    }
    pub fn write_color(&self, out: &mut File) {
        let intensity = Interval::new(0.0, 0.999);
        let rbyte = (intensity.clamp(linear_to_gamma(self.get_r())) * 256.0) as i32;
        let gbyte = (intensity.clamp(linear_to_gamma(self.get_g())) * 256.) as i32;
        let bbyte = (intensity.clamp(linear_to_gamma(self.get_b())) * 256.) as i32;
        writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).expect("Failed writing color!");
    }
}
pub fn write_to_png(filename: &str, image: &Vec<Vec<Vec3>>, width: i32, height: i32) {
    let mut encoder = ImageBuffer::new(width as u32, height as u32);

    println!("{}", image.len());
    for i in 0..height {
        for j in 0..width {
            let color = image[i as usize][j as usize];
            let rbyte = (linear_to_gamma(color.get_r()) * 256.0) as u8;
            let gbyte = (linear_to_gamma(color.get_g()) * 256.0) as u8;
            let bbyte = (linear_to_gamma(color.get_b()) * 256.0) as u8;
            encoder.put_pixel(j as u32, i as u32, image::Rgb([rbyte, gbyte, bbyte]));
        }
    }
    encoder.save(filename).unwrap();
}
fn linear_to_gamma(x: f64) -> f64 {
    if x > 0.0 {
        return x.sqrt();
    }
    0.0
}
