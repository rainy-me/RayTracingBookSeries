use rand::Rng;
use std::sync::Arc;
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

    pub fn to_color_string(self, samples_per_pixel: i32) -> String {
        let scale = 1.0 / samples_per_pixel as f64;

        vec![self.x, self.y, self.z]
            .iter()
            .map(|n| ((256.0 * clamp(n * scale, 0.0, 0.999)) as i32).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn random() -> Self {
        Vec3 {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }
    }

    pub fn random_in_range(low: f64, high: f64) -> Self {
        Vec3 {
            x: rand::thread_rng().gen_range(low, high),
            y: rand::thread_rng().gen_range(low, high),
            z: rand::thread_rng().gen_range(low, high),
        }
    }

    pub fn random_in_unit() -> Self {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
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

impl ops::AddAssign for Vec3<f64> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
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

impl ops::Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;
    fn mul(self, rhs: Vec3<f64>) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
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

    pub fn calc_color(self, hittable: &dyn Hittable, depth: i32) -> Color<f64> {
        // println!("depth: {}", depth);
        if depth <= 0 {
            return Color::from((0, 0, 0));
        }
        let mut record = HitRecord::default();
        if hittable.hit(&self, 0.0, std::f64::INFINITY, &mut record) {
            let target = record.point + record.normal + Vec3::random_in_unit();
            return Ray {
                origin: record.point,
                direction: target - record.point,
            }
            .calc_color(hittable, depth - 1)
                * 0.5;
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
    fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
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
    fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
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
        record.t = root;
        record.point = ray.at(record.t).clone();
        record.set_face_normal(ray, (record.point - self.center) / self.radius);
        true
    }
}

unsafe impl Send for HittableList {}
unsafe impl Sync for HittableList {}

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}
impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
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
    fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec;
            }
        }

        hit_anything
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::from((0, 0, 0));
        let horizontal = Vec3::from((viewport_width, 0.0, 0.0));
        let vertical = Vec3::from((0.0, viewport_height, 0.0));
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from((0.0, 0.0, focal_length));

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray<f64> {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match true {
        _ if x < min => min,
        _ if x > max => max,
        _ => x,
    }
}
