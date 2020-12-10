use crate::hittable::*;
use crate::vec3::*;
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn calc_color(self, hittable: &dyn Hittable, depth: i32) -> Color {
        // println!("depth: {}", depth);
        if depth <= 0 {
            return Color::from((0, 0, 0));
        }
        let mut record = HitRecord::default();
        if hittable.hit(&self, 0.001, std::f64::INFINITY, &mut record) {
            // let target = record.point + record.normal + Vec3::random_unit_vec();
            let target = record.point + record.normal.random_in_hemisphere();
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

    pub fn hit(&self, center: &Point3, radius: f64) -> f64 {
        hit_sphere(center, radius, self)
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Point3::default(),
            direction: Vec3::default(),
        }
    }
}

fn hit_sphere(&center: &Point3, radius: f64, ray: &Ray) -> f64 {
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
