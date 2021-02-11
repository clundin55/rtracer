use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(a: Color, fuzz: f32) -> Self {
        let mut f = 1.0;
        if fuzz < 1.0 {
            f = fuzz;
        }
        Self { albedo: a, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere().scale(self.fuzz),
        );
        *attenuation = self.albedo;
        return scattered.direction().dot(rec.normal) > 0.0;
    }
}
