use crate::{Color, Material, Point3, Ray, Vec3};

//TODO: document all fields
#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }

    pub fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
            material: Material::Metal {
                albedo: Color::new(),
                fuzz: 0.0,
            },
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn push(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord {
            p: Point3::from(0.0, 0.0, 0.0),
            normal: Vec3::from(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: Material::Metal {
                albedo: Color::new(),
                fuzz: 0.0,
            },
        };

        let mut hit_anything = false;
        let mut closest = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest, &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.t;
                *hit_record = temp_rec.clone();
            }
        }

        return hit_anything;
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn from(center: Point3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    /// Calculate the hit point of the ray on the spehere
    ///
    /// t^2 b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        // if the discriminant is greater than zero, that means our ray hits the sphere
        // at least once

        // (−b± √(b2−4ac) ) / (√2a)

        let oc = ray.origin() - self.center;

        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = ray.at(temp);
                let outward_normal = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(ray, outward_normal);
                hit_record.material = self.material;
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = ray.at(temp);
                let outward_normal = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(ray, outward_normal);
                hit_record.material = self.material;
                return true;
            }
        }
        return false;
    }
}
