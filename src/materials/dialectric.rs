use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

#[derive(Debug, Clone, Copy, Default)]
pub struct Dialectric {
    ir: f32, // Index of Refraction
}

impl Dialectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }
}

impl Material for Dialectric {
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut rng = rand::thread_rng();
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        }
        let unit_direction = Vec3::unit_vector(r.direction());
        let mut cos_theta = Vec3::dot(-unit_direction, rec.normal);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction;
        if (refraction_ratio * sin_theta > 1.0)
            || (reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0))
        {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        }
        *scattered = Ray::new(rec.p, direction);
        true
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
