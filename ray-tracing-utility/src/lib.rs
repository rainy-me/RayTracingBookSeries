use std::{convert, fmt, ops};

#[derive(Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// f64 only for now. TODO: change to <T: num> using crates?
impl Vec3<f64> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn mul(self, t: f64) -> Self {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    pub fn div(self, t: f64) -> Self {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }

    pub fn length_squared(self) -> f64 {
        self * self
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, target: Self) -> f64 {
        self * target
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
        self.div(len)
    }

    pub fn to_i32_vec(self) -> Vec3<i32> {
        Vec3 {
            x: (self.x * 256f64) as i32,
            y: (self.y * 256f64) as i32,
            z: (self.z * 256f64) as i32,
        }
    }
}

impl Vec3<i32> {
    pub fn print(self) {
        println!(
            "{}",
            vec![self.x, self.y, self.z,]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

impl Default for Vec3<f64> {
    fn default() -> Self {
        Vec3 {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }
}

impl convert::From<(i32, i32, i32)> for Vec3<f64> {
    fn from((a, b, c): (i32, i32, i32)) -> Self {
        Vec3 {
            x: a as f64,
            y: b as f64,
            z: c as f64,
        }
    }
}

impl convert::From<(f64, f64, f64)> for Vec3<f64> {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vec3 { x, y, z }
    }
}

impl fmt::Display for Vec3<f64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vec3({}, {},{})", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3<f64> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Neg for Vec3<f64> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub for Vec3<f64> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for Vec3<f64> {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
pub type Point3<T> = Vec3<T>;
pub type Color<T> = Vec3<T>;
pub struct Ray<T> {
    pub origin: Point3<T>,
    pub direction: Vec3<T>,
}

impl Ray<f64> {
    pub fn new(origin: Point3<f64>, direction: Vec3<f64>) -> Self {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3<f64> {
        self.origin + self.direction.mul(t)
    }
    pub fn color_vec(self) -> Color<f64> {
        let unit = self.direction.unit();
        let t = 0.5 * (unit.y + 1f64);
        return Color::from((1, 1, 1)).mul(1f64 - t) + Color::from((0.5f64, 0.7f64, 1f64)).mul(t);
    }
}

impl Default for Ray<f64> {
    fn default() -> Self {
        Ray {
            origin: Point3::default(),
            direction: Vec3::default(),
        }
    }
}
