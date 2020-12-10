use std::rc::Rc;
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

impl ops::Add<f64> for Vec3<f64> {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
impl ops::Add<Vec3<f64>> for Vec3<f64> {
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

#[derive(Copy, Clone)]
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

    pub fn calc_color(self, hittable: &dyn Hittable) -> Color<f64> {
        let mut hit_record = HitRecord::default();
        if hittable.hit(&self, 0.0, std::f64::INFINITY, &mut hit_record) {
            return (hit_record.normal + 1.0) * 0.5;
        }
        let unit = self.direction.unit();
        let t = 0.5 * (unit.y + 1f64);
        return Color::from((1, 1, 1)) * (1f64 - t) + Color::from((0.5f64, 0.7f64, 1f64)) * t;
    }

    pub fn hit(&self, center: &Point3<f64>, radius: f64) -> f64 {
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

fn hit_sphere(&center: &Point3<f64>, radius: f64, ray: &Ray<f64>) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc * ray.direction;
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant.is_sign_negative() {
        -1f64
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub point: Point3<f64>,
    pub normal: Vec3<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Vec3::from((0, 0, 0)),
            normal: Vec3::from((0, 0, 0)),
            t: 0f64,
            front_face: true,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray<f64>, outward_normal: Vec3<f64>) {
        let outward_normal = outward_normal.clone();
        self.front_face = (ray.direction * outward_normal).is_sign_negative();
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc * ray.direction;
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant.is_sign_negative() {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t).clone();
        hit_record.set_face_normal(ray, (hit_record.point - self.center) / self.radius);
        true
    }
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec;
            }
        }

        hit_anything
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}
