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
        self / len
    }

    pub fn to_color_string(self) -> String {
        vec![self.x, self.y, self.z]
            .iter()
            .map(|n| ((256f64 * n) as i32).to_string())
            .collect::<Vec<_>>()
            .join(" ")
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

impl ops::Mul<Vec3<f64>> for Vec3<f64> {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Mul<f64> for Vec3<f64> {
    type Output = Vec3<f64>;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f64> for Vec3<f64> {
    type Output = Vec3<f64>;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
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
        self.origin + self.direction * t
    }

    pub fn calc_color(self) -> Color<f64> {
        if self.hit(&Point3::from((0, 0, -1)), 0.5) {
            return Color::from((1, 0, 0));
        }
        let unit = self.direction.unit();
        let t = 0.5 * (unit.y + 1f64);
        return Color::from((1, 1, 1)) * (1f64 - t) + Color::from((0.5f64, 0.7f64, 1f64)) * t;
    }

    pub fn hit(&self, center: &Point3<f64>, radius: f64) -> bool {
        hit_sphere(center, radius, self)
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

fn hit_sphere(&center: &Point3<f64>, radius: f64, ray: &Ray<f64>) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction * ray.direction;
    let b = oc * ray.direction * 2f64;
    let c = oc * oc - radius * radius;
    let discriminant = b * b - 4f64 * a * c;
    discriminant > 0f64
}
