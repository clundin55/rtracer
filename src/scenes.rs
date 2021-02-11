use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::materials::*;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use rand::Rng;
use std::rc::Rc;

const PI: f32 = 3.1415926535897932385;

// Utility Functions
pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    };
    return x;
}

pub fn hit_sphere(center: &Point, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - (radius * radius);
    let discriminant = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - (discriminant).sqrt()) / a;
    }
}

pub fn create_world() {
    // Image
    let aspect_ratio = 3.0 / 20.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // Camera
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World

    let world: HittableList<Sphere> = end_image();
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        10.0,
        aspect_ratio,
        0.1,
        (lookfrom - lookat).length(),
    );

    println!("P3\n{} {} \n{}\n", image_width, image_height, 255);
    let mut color_vec: Vec<Color> = Vec::new();
    let mut rng = rand::thread_rng();

    ((0..image_height).rev()).into_iter().for_each(|j| {
        (0..image_width).into_iter().for_each(|i| {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let n1: f32 = rng.gen_range(0.0..1.0);
                let n2: f32 = rng.gen_range(0.0..1.0);
                let u = (i as f32 + n1) / (image_width as f32 - 1.0);
                let v = (j as f32 + n2) / (image_height as f32 - 1.0);
                let r: Ray = camera.get_ray(u, v);
                pixel_color += r.ray_color(&world, max_depth);
            }
            color_vec.push(pixel_color);
        })
    });
    color_vec
        .iter()
        .for_each(|pixel_color| pixel_color.write_color(samples_per_pixel));
}

pub fn end_image() -> HittableList<Sphere> {
    let mut rng = rand::thread_rng();
    let objects: Vec<Sphere> = Vec::new();
    let mut world = HittableList::new(objects);
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    (-11..11).into_iter().for_each(|a| {
        (-11..11).into_iter().for_each(|b| {
            let choose_material = rng.gen_range(0.0..1.0);
            let center = Point::new(
                a as f32 + (0.9 * rng.gen_range(0.0..1.0)),
                0.2,
                b as f32 + (0.9 * rng.gen_range(0.0..1.0)),
            );
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    world.add(Sphere::new(center, 0.2, Rc::new(Lambertian::new(albedo))));
                } else if choose_material < 0.95 {
                    // metal
                    //let albedo = Color::random(0.5, 1);
                    let albedo = Color::random();
                    let fuzz = rng.gen_range(0.0..0.5);
                    world.add(Sphere::new(center, 0.2, Rc::new(Metal::new(albedo, fuzz))));
                } else {
                    // glass
                    world.add(Sphere::new(center, 0.2, Rc::new(Dialectric::new(1.5))));
                }
            }
        })
    });
    world.add(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dialectric::new(1.5)),
    ));

    world.add(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    ));

    world.add(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    ));

    world
}
