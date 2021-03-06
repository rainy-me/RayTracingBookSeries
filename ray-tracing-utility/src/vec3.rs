use crate::*;
use std::{convert, fmt, ops};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// f64 only for now. TODO: change to <T: num> using crates?
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, target: Self) -> f64 {
        self.x * target.x + self.y * target.y + self.z * target.z
    }

    pub fn cross(self, target: Self) -> Self {
        Vec3 {
            x: self.y * target.z - self.z * target.y,
            y: self.z * target.x - self.x * target.z,
            z: self.x * target.y - self.y * target.x,
        }
    }

    pub fn unit(self) -> Self {
        let len = self.length();
        self / len
    }

    pub fn rgb(r: i32, g: i32, b: i32) -> Self {
        Vec3 {
            x: r as f64 / 255.999,
            y: g as f64 / 255.999,
            z: b as f64 / 255.999,
        }
    }

    pub fn to_color_string(self, samples_per_pixel: i32) -> String {
        let scale = 1. / samples_per_pixel as f64;

        vec![self.x, self.y, self.z]
            .iter()
            .map(|n| ((255.999 * clamp((n * scale).sqrt(), 0., 0.999)) as i32).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn random() -> Self {
        Vec3 {
            x: rand_f64(),
            y: rand_f64(),
            z: rand_f64(),
        }
    }

    pub fn random_in_range(low: f64, high: f64) -> Self {
        Vec3 {
            x: rand_f64_in_range(low, high),
            y: rand_f64_in_range(low, high),
            z: rand_f64_in_range(low, high),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_in_range(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vec() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemisphere(self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if (in_unit_sphere.dot(self)).is_sign_positive() {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(self) -> bool {
        const SMALL: f64 = 1e-8;
        (self.x < SMALL) && (self.y < SMALL) && (self.z < SMALL)
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - self.dot(normal) * normal * 2.
    }

    pub fn refract(self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
        r_out_perp + r_out_parallel
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }
}

impl convert::From<(i32, i32, i32)> for Vec3 {
    fn from((a, b, c): (i32, i32, i32)) -> Self {
        Vec3 {
            x: a as f64,
            y: b as f64,
            z: c as f64,
        }
    }
}

impl convert::From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vec3 { x, y, z }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vec3({}, {},{})", self.x, self.y, self.z)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from((self.x * rhs.x, self.y * rhs.y, self.z * rhs.z))
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match true {
        _ if x < min => min,
        _ if x > max => max,
        _ => x,
    }
}
