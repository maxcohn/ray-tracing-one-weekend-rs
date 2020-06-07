
use crate::{Vec3, Color, Point3, Ray, HitRecord, Hittable};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Metal {
        albedo: Color,
        fuzz: f64,
    },
    Lambertian { albedo: Color },
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match &self {
            Material::Metal { albedo, fuzz } => {
                let real_fuzz = if *fuzz >= 1.0 {
                    1.0
                } else {
                    *fuzz
                };

                let reflected = ray_in.direction().unit_vector().reflect(rec.normal);
                *scattered = Ray::from(rec.p, reflected + real_fuzz * Vec3::random_in_unit_sphere());
                *attenuation = *albedo;

                scattered.direction().dot(rec.normal) > 0.0
            },
            Material::Lambertian { albedo } => {
                let scatter_dir = rec.normal + Vec3::random_unit_vector();
                *scattered = Ray::from(rec.p, scatter_dir);
                *attenuation = *albedo;

                true
            },
        }
    }
}
