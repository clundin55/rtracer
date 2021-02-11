use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, mat_ptr: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center).divide(self.radius);
        rec.mat_ptr = Some(self.mat_ptr.clone());
        let outward_normal: Vec3 = (rec.p - self.center).divide(self.radius);
        rec.set_face_normal(r, &outward_normal);
        true
    }
}
