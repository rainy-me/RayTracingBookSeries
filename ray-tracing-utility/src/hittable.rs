use crate::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Vec3::from((0, 0, 0)),
            normal: Vec3::from((0, 0, 0)),
            t: 0f64,
            front_face: true,
            material: Arc::new(Lambertian::new(0, 0, 0)),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = (ray.direction.dot(outward_normal)).is_sign_negative();
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius,
            material: Arc::new(Lambertian::new(0, 0, 0)),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
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
        record.material = self.material.clone();
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec.clone();
            }
        }

        hit_anything
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.;
}
