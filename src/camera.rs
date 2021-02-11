use crate::point::Point;
use crate::ray::Ray;
use crate::scenes::degrees_to_radians;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u.scale(viewport_width).scale(focus_dist);
        let vertical = v.scale(viewport_height).scale(focus_dist);

        let lower_left_corner =
            origin - horizontal.divide(2.0) - vertical.divide(2.0) - w.scale(focus_dist);
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk().scale(self.lens_radius);
        let offset = self.u.scale(rd.x) + self.v.scale(rd.y);
        Ray::new(
            self.origin + offset,
            (self.lower_left_corner + self.horizontal.scale(s))
                + (self.vertical.scale(t) - self.origin - offset),
        )
    }
}
