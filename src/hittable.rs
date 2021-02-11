use crate::materials::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat_ptr: Option<Rc<dyn Material>>,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
}

impl<T> HittableList<T>
where
    T: Hittable,
{
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, new_object: T) {
        self.objects.push(new_object);
    }
    pub fn new(objects: Vec<T>) -> HittableList<T> {
        HittableList { objects }
    }
}

impl<T> Hittable for HittableList<T>
where
    T: Hittable,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord {
            p: Point::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            mat_ptr: None,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for x in &self.objects {
            if x.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(*outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
            return;
        }
        self.normal = -(*outward_normal);
    }
}
