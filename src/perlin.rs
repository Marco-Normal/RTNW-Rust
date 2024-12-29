use crate::vec3::{random_unit_vector, Point3, Vec3};
use rand::Rng;
#[derive(Clone, Default)]
pub struct Perlin {
    point_count: usize,
    randfloat: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Self {
        let randfloat = (0..point_count)
            .map(|_| Vec3::random_range(-1.0, 1.0))
            .collect();
        Perlin {
            point_count,
            randfloat,
            perm_x: Perlin::perlin_generate_per(point_count),
            perm_y: Perlin::perlin_generate_per(point_count),
            perm_z: Perlin::perlin_generate_per(point_count),
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - f64::floor(p.x());
        let v = p.y() - f64::floor(p.y());
        let w = p.z() - f64::floor(p.z());
        let i = f64::floor(p.x());
        let j = f64::floor(p.y());
        let k = f64::floor(p.z());
        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[self.perm_x
                        [(i + di as f64) as usize & (self.point_count - 1)]
                        ^ self.perm_y[(j + dj as f64) as usize & (self.point_count - 1)]
                        ^ self.perm_z[(k + dk as f64) as usize & (self.point_count - 1)]];
                }
            }
        }
        Perlin::perlin_interp(&c, u, v, w)
    }
    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k].dot_product(&weight);
                }
            }
        }
        accum
    }

    fn perlin_generate_per(point_count: usize) -> Vec<usize> {
        let mut p: Vec<usize> = (0..point_count).collect();
        let mut rng = rand::thread_rng();
        for i in (0..point_count).rev() {
            let target = rng.gen_range(0..i + 1);
            p.swap(i, target);
        }
        p
    }
    pub fn turbulence(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        f64::abs(accum)
    }
}
