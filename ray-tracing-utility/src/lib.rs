pub mod camera;
pub mod hittable;
pub mod materials;
pub mod ray;
pub mod vec3;

pub use camera::*;
pub use hittable::*;
pub use materials::*;
pub use ray::*;
pub use vec3::*;

use rand::Rng;

pub fn rand_f64() -> f64 {
    rand::random::<f64>()
}

pub fn rand_f64_in_range(low: f64, high: f64) -> f64 {
    rand::thread_rng().gen_range(low, high)
}
