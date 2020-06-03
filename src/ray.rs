

use crate::{Vec3, Point3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn from(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> Point3{
        self.origin
    }

    pub fn direction(&self) -> Vec3{
        self.direction
    }

    /// Calculate the position of the ray at the given time
    pub fn at(&self, t: f64) -> Point3 {
        // P(t) = A + tB
        //
        // Where P = 3D position
        // A = Ray origin
        // B = Ray direction
        // t = time
        self.origin + self.direction * t
    }
}