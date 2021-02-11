use rand::Rng;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
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
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
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
    pub fn new_random(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(min..max);
        let y: f32 = rng.gen_range(min..max);
        let z: f32 = rng.gen_range(min..max);
        Vec3 { x, y, z }
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::new_random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere()
    }

    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn scale(self, t: f32) -> Vec3 {
        Self {
            x: (self.x * t),
            y: (self.y * t),
            z: (self.z * t),
        }
    }
    pub fn divide(self, t: f32) -> Vec3 {
        Self {
            x: (self.x / t),
            y: (self.y / t),
            z: (self.z / t),
        }
    }
    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        *v - n.scale(2.0 * v.dot(*n))
    }
    pub fn dot(self, other: Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    pub fn cross(u: &Vec3, v: &Vec3) -> Self {
        Self {
            x: (u.y * v.z - u.z * v.y),
            y: (u.z * v.x - u.x * v.z),
            z: (u.x * v.y - u.y * v.x),
        }
    }
    pub fn unit_vector(self) -> Self {
        self.divide(self.length())
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let mut cos_theta = (-(*uv)).dot(*n);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let q = (*n).scale(cos_theta);
        let r_out_perp = (*uv + q).scale(etai_over_etat);
        let inner = (1.0 - r_out_perp.length_squared()).abs().sqrt();
        let r_out_parallel = n.scale(-inner);
        r_out_perp + r_out_parallel
    }
}
