use std::f64;
use std::ops;

use rand::Rng;

use crate::util;

/// A collections of three points representing a location in 3D space.
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    /// Create a new Vec3 with all values set to 0
    pub fn new() -> Self {
        Vec3 {
            e: [0f64, 0f64, 0f64],
        }
    }

    /// Create a new Vec3 with the given values
    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    /// Get the X coordinate of the Vec3
    #[inline]
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    /// Get the Y coordinate of the Vec3
    #[inline]
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    /// Get the Z coordinate of the Vec3
    #[inline]
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// Print the Vec3 to stdout
    pub fn print(&self) {
        println!("{} {} {}", self.x(), self.y(), self.z());
    }

    /// Print the Vec3 as a color to stdout
    pub fn print_color(&self, samples_per_pixel: u32) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // divide the color total by the number of samples
        let scale = 1.0 / samples_per_pixel as f64;

        // We sqrt so as a gamma adjust. We raise the color to the power (1/gamma)
        // and we're using a gamma of 2, thus a sqrt
        r = (r * scale).sqrt();
        b = (b * scale).sqrt();
        g = (g * scale).sqrt();

        println!(
            "{} {} {}",
            (256.0 * util::clamp(r, 0.0, 0.999)) as i32,
            (256.0 * util::clamp(g, 0.0, 0.999)) as i32,
            (256.0 * util::clamp(b, 0.0, 0.999)) as i32
        );
    }

    #[inline]
    pub fn dot(&self, v: Self) -> f64 {
        (self.e[0] * v.e[0]) + (self.e[1] * v.e[1]) + (self.e[2] * v.e[2])
    }

    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self::from(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    /// Generate a random Vec3 in with values between 0.0 and 1.0
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::from(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
    }

    /// Generate a random Vec3 in with values in the given range
    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self::from(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    pub fn random_unit_vector() -> Self {
        // https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials/truelambertianreflection
        // Lambertian distribution. We chose this distribution because it is more
        // uniform. We do this by choosing points on the surface of the unit sphere
        // offset along the surface normal
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen_range(0.0, 2.0 * f64::consts::PI);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();

        Self::from(r * a.cos(), r * a.sin(), z)
    }

    pub fn reflect(self, other: Vec3) -> Vec3 {
        self - 2.0 * self.dot(other) * other
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::from(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);

            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(n);
        let r_out_parallel = etai_over_etat * (self + cos_theta * n);
        let r_out_perpendicular = (-(1.0 - r_out_parallel.length_squared()).sqrt()) * n;

        r_out_parallel + r_out_perpendicular
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [self.x() + other, self.y() + other, self.z() + other],
        }
    }
}
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1f64 / other
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Self::from(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Self::from(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        Self::from(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Self::Output {
        t * self //Self::from(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, t: Vec3) -> Self::Output {
        Vec3::from(t.x() * self, t.y() * self, t.z() * self)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Self::Output {
        (1.0 / t) * self
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
