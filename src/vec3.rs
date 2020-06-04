use std::ops;

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

    /// Print the Vec3 as a color
    pub fn print_color(&self, samples_per_pixel: usize) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // divide the color total by the number of samples
        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        b *= scale;
        g *= scale;

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
            -(self.e[2] * other.e[0] - self.e[0] * other.e[2]),
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
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
