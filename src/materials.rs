use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

mod dialectric;
mod lambertian;
mod metal;
pub use dialectric::Dialectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material {
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
