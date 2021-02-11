use crate::clamp;
use crate::vec3::Vec3;
use rand::Rng;

pub type Color = Vec3;

impl Color {
    pub fn new(x: f32, y: f32, z: f32) -> Color {
        Self { x, y, z }
    }
    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0.0..1.0),
            y: rng.gen_range(0.0..1.0),
            z: rng.gen_range(0.0..1.0),
        }
    }

    pub fn write_color(&self, samples_per_pixel: u32) {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;
        // Divide the color by the number of samples.
        let scale = 1.0 / samples_per_pixel as f32;
        r *= scale;
        g *= scale;
        b *= scale;

        println!(
            "{} {} {}\n",
            (256.0 * clamp(r.sqrt(), 0.0, 0.999)) as u32,
            (256.0 * clamp(g.sqrt(), 0.0, 0.999)) as u32,
            (256.0 * clamp(b.sqrt(), 0.0, 0.999)) as u32
        );
    }
}
