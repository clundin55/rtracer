use crate::color::Color;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::point::Point;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

#[derive(Copy, Debug, Clone, Default)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction.scale(t)
    }
    pub fn ray_color(&self, world: &HittableList<Sphere>, depth: u32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::default();
        }
        let mut rec: HitRecord = HitRecord::default();

        if world.hit(self, 0.001, f32::INFINITY, &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if let Some(ref mat_ptr) = rec.mat_ptr {
                if mat_ptr.scatter(&self, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * scattered.ray_color(world, depth - 1);
                }
            }
            return Color::default();
        }
        let unit_direction: Vec3 = self.direction().unit_vector();
        let t = (unit_direction.y + 1.0) * 0.5;
        let c1 = Color::new(1.0, 1.0, 1.0);
        let c2 = Color::new(0.5, 0.7, 1.0);
        c1.scale(1.0 - t) + c2.scale(t)
    }
}
