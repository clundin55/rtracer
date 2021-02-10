use std::ops::{Add, Div, Mul, Sub};

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

struct HitRecord {
    p: Point,
    normal: Vec3,
    t: f32,
}

trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: (self.x - other.x),
            y: (self.y - other.y),
            z: (self.z - other.z),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: (self.x + other.x),
            y: (self.y + other.y),
            z: (self.z + other.z),
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: (self.x * other.x),
            y: (self.y * other.y),
            z: (self.z * other.z),
        }
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: (self.x / other.x),
            y: (self.y / other.y),
            z: (self.z / other.z),
        }
    }
}

impl Vec3 {
    fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    fn scale(self, t: f32) -> Vec3 {
        Self {
            x: (self.x * t),
            y: (self.y * t),
            z: (self.z * t),
        }
    }
    fn divide(self, t: f32) -> Vec3 {
        Self {
            x: (self.x / t),
            y: (self.y / t),
            z: (self.z / t),
        }
    }
    fn dot(self, other: Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    fn unit_vector(self) -> Self {
        self.divide(self.length())
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Color {
    fn new(x: f32, y: f32, z: f32) -> Color {
        Self { x: x, y: y, z: z }
    }

    fn write_color(&self) {
        let ir = (255.999 * self.x) as u32;
        let ig = (255.999 * self.y) as u32;
        let ib = (255.999 * self.z) as u32;
        println!("{} {} {}\n", ir, ig, ib);
    }
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction.scale(t)
    }
    pub fn ray_color(&self) -> Color {
        let t = hit_sphere(
            &Point {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            self,
        );
        if t > 0.0 {
            let n = (self.at(t)
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                })
            .unit_vector();
            return Color {
                x: n.x + 1.0,
                y: n.y + 1.0,
                z: n.z + 1.0,
            }
            .scale(0.5);
        }
        let unit_direction: Vec3 = self.direction().unit_vector();

        let t = (unit_direction.y + 1.0) * 0.5;
        let c1 = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let c2 = Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };
        c1.scale(1.0 - t) + c2.scale(t)
    }
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

pub fn create_ppm() {
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f32) / (IMAGE_WIDTH as f32 - 1.0);
            let g = (j as f32) / (IMAGE_HEIGHT as f32 - 1.0);
            let b = 0.25;

            let pixel = Color::new(r, g, b);
            pixel.write_color();
        }
    }
}

pub fn create_image() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal.divide(2.0)
        - vertical.divide(2.0)
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };
    println!("P3\n{} {} \n{}\n", image_width, image_height, 255);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = (i as f32) / (image_width as f32 - 1.0);
            let v = (j as f32) / (image_height as f32 - 1.0);
            let r: Ray = Ray {
                origin,
                direction: (lower_left_corner + horizontal.scale(u)) + (vertical.scale(v) - origin),
            };
            let pixel = r.ray_color();
            pixel.write_color();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    #[test]
    fn create_ppm() -> std::io::Result<()> {
        println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

        for j in (0..IMAGE_HEIGHT).rev() {
            for i in 0..IMAGE_WIDTH {
                let r = (i as f32) / (IMAGE_WIDTH as f32 - 1.0);
                let g = (j as f32) / (IMAGE_HEIGHT as f32 - 1.0);
                let b = 0.25;

                let pixel = Color::new(r, g, b);
                pixel.write_color();
            }
        }
        Ok(())
    }
}
