use crate::color::Color;
use crate::common::random_double;
use crate::hittable::HitRecord;
use crate::rays::Ray;
use crate::textures::Texture;
use crate::vec3;
use crate::vec3::{random_unit_vector, reflect, refract};
use crate::vec3::{Point3, Vec3};

pub struct Lambertian<T: Texture> {
    albedo: T,
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

pub struct Dielectric {
    refraction_index: f64,
}

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub struct DiffuseLight<T: Texture> {
    texture: T,
}

pub struct Isotropic<T: Texture> {
    texture: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction: Vec3 = rec.normal() + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }
        let scatter_record = ScatterRecord {
            attenuation: self.albedo.value(rec.u(), rec.v(), &rec.p()),
            scattered: Ray::new(rec.p(), scatter_direction, ray_in.time()),
        };
        Some(scatter_record)
    }
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Metal {
            albedo: color,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected: Vec3 = reflect(&ray_in.direction().normalize(), &rec.normal()).normalize()
            + (self.fuzz * random_unit_vector());
        let scatter_record = ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p(), reflected, ray_in.time()),
        };
        if scatter_record
            .scattered
            .direction()
            .dot_product(&rec.normal())
            > 0.0
        {
            Some(scatter_record)
        } else {
            None
        }
    }
}

impl Dielectric {
    pub fn new(index: f64) -> Self {
        Dielectric {
            refraction_index: index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction().normalize();
        let cos_theta = f64::min(-unit_direction.dot_product(&rec.normal()), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || vec3::reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(&unit_direction, &rec.normal())
            } else {
                refract(&unit_direction, &rec.normal(), refraction_ratio)
            };
        let scatter_record = ScatterRecord {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(rec.p(), direction, ray_in.time()),
        };

        Some(scatter_record)
    }
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(texture: T) -> Self {
        DiffuseLight { texture }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn emmited(&self, p: &Point3, u: f64, v: f64) -> Color {
        self.texture.value(u, v, p)
    }
}

impl<T: Texture> Isotropic<T> {
    pub fn new(texture: T) -> Self {
        Isotropic { texture }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.texture.value(rec.u(), rec.v(), &rec.p()),
            scattered: Ray::new(rec.p(), random_unit_vector(), ray_in.time()),
        })
    }
    fn emmited(&self, _p: &Point3, _u: f64, _v: f64) -> Color {
        Color::default()
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn emmited(&self, p: &Point3, u: f64, v: f64) -> Color {
        Color::default()
    }
}
