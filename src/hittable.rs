use crate::{Vec3, Point3, Color, Ray}

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: double,
}

trait Hittable {
    fn hit(ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}


pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn from(center: Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    /// Calculate the hit point of the ray on the spehere
    ///
    /// t^2 b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0
    fn hit(ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        // if the discriminant is greater than zero, that means our ray hits the sphere
        // at least once
        
        // (−b± √(b2−4ac) ) / (√2a)

        let oc = ray.origin() - center;
        
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - radius*radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = ray.at(temp);
                hit_record.normal = (hit_record.p - center) / radius;
                return true;
            }
            let temp = (-half_b + root) / a;
            if (temp < t_max && temp > t_min) {
                hit_record.t = temp;
                hit_record.p = ray.at(temp);
                hit_record.normal = (hit_record.p - center) / radius;
                return true;
            }
        }
        return false;
    }
}