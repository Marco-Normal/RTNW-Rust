use crate::color::write_to_png;

use super::{
    color::Color,
    common::{degree_to_radians, random_double, INFINITY},
    hittable::Hittable,
    interval::Interval,
    rays::Ray,
    vec3::{random_on_disk, Point3, Vec3},
};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::cmp;
/// Definition of a camera. The camera is defined by the following parameters:
/// - Aspect ratio
/// - Image width
/// - Image height
/// - Samples per pixel: Number of vectors casted per pixel
/// - Pixel sample scale: 1 / samples per pixel
/// - Center: Center of the camera
/// - Pixel 00 location: Location where the first pixel is located
/// - Delta u: Vector that represents the change in the u direction
/// - Delta v: Vector that represents the change in the v direction
/// - Max depth: Maximum depth of the ray
/// - Vfov: Vertical field of view
/// - Lookfrom: Point where the camera is looking from
/// - Lookat: Point where the camera is looking at
/// - Vup: Up direction of the camera
/// - U: U vector of the camera. Component of the orthonormal basis of the camera
/// - V: V vector of the camera. Component of the orthonormal basis of the camera
/// - W: W vector of the camera. Component of the orthonormal basis of the camera
/// - Defocus angle: Angle of the defocus disk
/// - Focus distance: Distance of the focus plane
/// - Defocus disk u: U vector of the defocus disk
/// - Defocus disk v: V vector of the defocus disk
/// - Background: Color of the background of the scene
#[derive(Default)]
pub struct Camera {
    aspect_ratio: Option<f64>,
    image_width: Option<i32>,
    image_height: i32,
    samples_per_pixel: i32,
    pixel_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    delta_u: Vec3,
    delta_v: Vec3,
    max_depth: Option<i32>,
    vfov: Option<f64>,
    lookfrom: Option<Point3>,
    lookat: Option<Point3>,
    vup: Option<Vec3>,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: Option<f64>,
    focus_distance: Option<f64>,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    background: Option<Color>,
}

impl Camera {
    /// Initializes the camera with the default values. If some of the values are not set, it will
    /// print a warning message and use the default values. The default values are:
    /// - Vfov: 90 degrees
    /// - Lookfrom: (0,0,0)
    /// - Lookat: (0,0,-1)
    /// - Vup: (0,1,0)
    /// - Focus distance: 10.0
    /// - Defocus angle: 0.0
    /// - Image width: 800
    /// - Max depth: 50
    /// - Aspect ratio: 16:9
    /// - Background: 0,0,0
    /// - Other values are calculated based on the previous values
    fn initialize(&mut self) {
        if self.vfov.is_none() {
            eprintln!("No vertical field of view set, using default 90 degrees");
            self.vfov = Some(90.0);
        }
        if self.lookfrom.is_none() {
            eprintln!("No lookfrom point set, using default (0,0,0)");
            self.lookfrom = Some(Default::default());
        }
        if self.lookat.is_none() {
            eprintln!("No lookat point set, using default (0,0,-1)");
            self.lookat = Some(Point3::new(0.0, 0.0, -1.0));
        }
        if self.vup.is_none() {
            eprintln!("No default up direction set, using default (0,1,0)");
            self.vup = Some(Vec3::new(0.0, 1.0, 0.0));
        }
        if self.focus_distance.is_none() {
            eprintln!("No focus distance set, using default 10.0");
            self.focus_distance = Some(10.0);
        }
        if self.defocus_angle.is_none() {
            eprintln!("No defocus angle set, using default 0.0");
            self.defocus_angle = Some(0.0);
        }
        if self.image_width.is_none() {
            eprintln!("No image width set, using default 800");
            self.image_width = Some(800);
        }
        if self.max_depth.is_none() {
            eprintln!("No max depth set, using default 50");
            self.max_depth = Some(50);
        }
        if self.aspect_ratio.is_none() {
            eprintln!("No aspect ratio set, using default 16:9");
            self.aspect_ratio = Some(16.0 / 9.0);
        }
        if self.background.is_none() {
            eprintln!("No background color set, using the default pure black");
            self.background = Some(Color::default());
        }

        // Image
        self.image_height = cmp::max(
            (self.image_width.unwrap() as f64 / self.aspect_ratio.unwrap()) as i32,
            1,
        );
        // Camera
        let theta = degree_to_radians(self.vfov.unwrap());
        let h = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h * self.focus_distance.unwrap();
        let viewport_width: f64 =
            viewport_height * (self.image_width.unwrap() as f64 / self.image_height as f64);
        self.center = self.lookfrom.unwrap();
        // Basis
        self.w = (self.lookfrom.unwrap() - self.lookat.unwrap()).normalize();
        self.u = self.vup.unwrap().cross_product(&self.w).normalize();
        self.v = self.w.cross_product(&self.u);
        // Aux vectors
        let viewport_u: Vec3 = viewport_width * self.u;
        let viewport_v: Vec3 = viewport_height * -self.v;
        // Pixel delta
        self.delta_u = viewport_u / self.image_width.unwrap() as f64;
        self.delta_v = viewport_v / self.image_height as f64;
        // Calculate the 00 pixel
        let viewport_upper_left = self.center
            - (self.focus_distance.unwrap() * self.w)
            - viewport_u / 2.
            - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.delta_u + self.delta_v);
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;
        // Calculate the defocus disk
        let defocus_radius = self.focus_distance.unwrap()
            * f64::tan(degree_to_radians(self.defocus_angle.unwrap() / 2.0));
        // Calculate the defocus disk vectors
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    /// Renders the image using the camera. The image is rendered using the following steps:
    /// - Initialize the camera
    /// - Open the file
    /// - Write the header of the file
    /// - For each pixel in the image:
    ///    - Calculate the pixel color
    ///    - Write the pixel color to the file
    ///    - Repeat for all samples per pixel
    ///    - Repeat for all pixels
    ///    - Close the file
    ///    - Print a message when the image is done
    pub fn render(&mut self, world: &Box<dyn Hittable>, filename: String) {
        self.initialize();

        // let mut file = File::create(filename).expect("Couldn't Open file");
        // let header = format!(
        //     "P3\n {} {} \n255\n",
        //     self.image_width.unwrap(),
        //     self.image_height
        // );
        // write!(file, "{}", header).expect("Couldn't write to file");
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("#>-");
        let bar = ProgressBar::new(self.image_height as u64);
        bar.set_style(sty);
        bar.set_message("Rendering image...");
        let image: Vec<Vec<_>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                bar.inc(1);
                let pixel_colors: Vec<_> = (0..self.image_width.unwrap())
                    .into_par_iter()
                    .map(|i| {
                        let mut pixel_color = Color::default();
                        for _ in 0..self.samples_per_pixel {
                            let ray: Ray = self.get_ray(i, j);
                            pixel_color += self.ray_color(&ray, world, self.max_depth.unwrap());
                        }
                        pixel_color * self.pixel_sample_scale
                    })
                    .collect();
                pixel_colors
            })
            .collect();
        write_to_png(
            &filename,
            &image,
            self.image_width.unwrap(),
            self.image_height,
        );
        bar.finish_with_message("\nRendering Done!!\n");
    }
    /// Returns the ray that goes from the camera to the pixel (i,j). The ray is calculated using
    /// the following steps:
    /// - Calculate the offset of the pixel. It is based on a 1 x 1 square, where we randomly sample from it
    /// - Calculate the pixel sample. It is calculated by adding the offset to the pixel 00 location
    /// - Calculate the ray origin. If the defocus angle is less than or equal to 0, the ray origin is the center of the camera
    /// - Calculate the ray direction. It is calculated by subtracting the pixel sample from the ray origin
    /// - Return the ray
    ///
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.delta_u + ((j as f64 + offset.y()) * self.delta_v));
        let ray_origin = if self.defocus_angle.unwrap() <= 0.0 {
            self.center
        } else {
            self.sample_disk()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();
        Ray::new(ray_origin, ray_direction, ray_time)
    }
    /// Samples a point in the defocus disk. The point is sampled using the following steps:
    /// - Sample a point in the disk
    /// - Calculate the point in the defocus disk by adding the point in the disk to the defocus disk vectors
    /// - Return the point in the defocus disk
    fn sample_disk(&self) -> Vec3 {
        let p = random_on_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.)
    }
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = Some(aspect_ratio);
    }
    pub fn set_width(&mut self, width: i32) {
        self.image_width = Some(width);
    }
    pub fn set_sample_per_pixel(&mut self, samples: i32) {
        self.samples_per_pixel = samples;
    }
    pub fn set_max_depth(&mut self, depth: i32) {
        self.max_depth = Some(depth);
    }
    pub fn set_vertical_fov(&mut self, vfov: f64) {
        self.vfov = Some(vfov);
    }
    pub fn set_lookfrom(&mut self, from: Point3) {
        self.lookfrom = Some(from);
    }
    pub fn set_lookat(&mut self, at: Point3) {
        self.lookat = Some(at);
    }
    pub fn set_vup(&mut self, vup: Vec3) {
        self.vup = Some(vup);
    }
    pub fn set_defocus_angle(&mut self, angle: f64) {
        self.defocus_angle = Some(angle);
    }
    pub fn set_focus_distance(&mut self, distance: f64) {
        self.focus_distance = Some(distance);
    }
    pub fn set_background_color(&mut self, color: Color) {
        self.background = Some(color);
    }
    /// Calculates the color of the ray. The color is calculated using the following steps:
    /// - If the depth is less than or equal to 0, return the default color
    /// - If the ray intersects with an object:
    ///   - If the object scatters the ray, calculate the scattered ray and the attenuation
    ///   - Return the attenuation multiplied by the color of the scattered ray
    ///   - If the object does not scatter the ray, return the default color
    ///   - If the ray does not intersect with an object, calculate the background color
    ///   - Return the background color
    pub fn ray_color(&self, ray: &Ray, world: &Box<dyn Hittable>, depth: i32) -> Color {
        if depth <= 0 {
            return Color::default();
        }
        // Hack for floating point inacuracies. If the hit is super close to the
        // already intersected point, ignore it. Get rid of shadow acne
        let time_interval = Interval::new(0.001, INFINITY);
        if let Some(rec) = world.hit(ray, &time_interval) {
            let color_from_emission =
                rec.get_material()
                    .unwrap()
                    .emmited(&rec.p(), rec.u(), rec.v());
            if let Some(scatter_rec) = rec.get_material().as_ref().unwrap().scatter(ray, &rec) {
                return color_from_emission
                    + scatter_rec.attenuation
                        * self.ray_color(&scatter_rec.scattered, world, depth - 1);
            }
            return color_from_emission;
        }
        self.background.unwrap()
    }
}
/// Calculates the color of the ray. The color is calculated using the following steps:
/// - If the depth is less than or equal to 0, return the default color
/// - If the ray intersects with an object:
///   - If the object scatters the ray, calculate the scattered ray and the attenuation
///   - Return the attenuation multiplied by the color of the scattered ray
///   - If the object does not scatter the ray, return the default color
///   - If the ray does not intersect with an object, calculate the background color
///   - Return the background color
fn ray_color(ray: &Ray, world: &Box<dyn Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    // Hack for floating point inacuracies. If the hit is super close to the
    // already intersected point, ignore it. Get rid of shadow acne
    let time_interval = Interval::new(0.001, INFINITY);
    if let Some(rec) = world.hit(ray, &time_interval) {
        if let Some(scatter_rec) = rec.get_material().as_ref().unwrap().scatter(ray, &rec) {
            return scatter_rec.attenuation * ray_color(&scatter_rec.scattered, world, depth - 1);
        }
        return Color::default();
    }
    let unit_vector: Vec3 = ray.direction().normalize();
    let a: f64 = 0.5 * (unit_vector.y() + 1.0);
    (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0)
}
