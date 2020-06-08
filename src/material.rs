use rand::Rng;

use crate::{Color, HitRecord, Hittable, Point3, Ray, Vec3};



#[derive(Debug, Clone, Copy)]
pub enum Material {
    Metal { albedo: Color, fuzz: f64 },
    Lambertian { albedo: Color },
    Dielectric { ref_idx: f64 },
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match &self {
            Material::Metal { albedo, fuzz } => {
                let real_fuzz = if *fuzz >= 1.0 { 1.0 } else { *fuzz };

                let reflected = ray_in.direction().unit_vector().reflect(rec.normal);
                *scattered =
                    Ray::from(rec.p, reflected + real_fuzz * Vec3::random_in_unit_sphere());
                *attenuation = *albedo;

                scattered.direction().dot(rec.normal) > 0.0
            }
            Material::Lambertian { albedo } => {
                let scatter_dir = rec.normal + Vec3::random_unit_vector();
                *scattered = Ray::from(rec.p, scatter_dir);
                *attenuation = *albedo;

                true
            }
            Material::Dielectric { ref_idx } => {
                let mut rng = rand::thread_rng();


                *attenuation = Color::from(1.0, 1.0, 1.0);

                // calculate if the light should refract or not
                let etai_over_etat = if rec.front_face {
                    1.0 / *ref_idx
                } else {
                    *ref_idx
                };

                let unit_dir = ray_in.direction().unit_vector();

                let cos_theta = (-unit_dir).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                if etai_over_etat * sin_theta > 1.0 {
                    // Reflect
                    let reflected = unit_dir.reflect(rec.normal);
                    *scattered = Ray::from(rec.p, reflected);
                    return true;
                }

                let reflect_prob = schlick(cos_theta, etai_over_etat);
                
                if rng.gen::<f64>() < reflect_prob {
                    let reflected = unit_dir.reflect(rec.normal);
                    *scattered = Ray::from(rec.p, reflected);
                    return true;
                }

                // Refract
                let refracted = unit_dir.refract(rec.normal, etai_over_etat);
                *scattered = Ray::from(rec.p, refracted);

                true
            }
        }
    }
}

/// Schlick approximation baed on the cosine and refraction index
///
/// R0 = ((n1 - n2)/(n1 + n2))^2
/// R(theta) = R0 + (1 - R0)(1 - cos(theta))^5
fn schlick(cos: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
