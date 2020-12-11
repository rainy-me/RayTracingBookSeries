use crate::*;
use std::sync::Arc;
pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self {
            albedo: Color::rgb(r, g, b),
        }
    }

    pub fn as_ref(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + Vec3::random_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }
        *scattered = Ray {
            origin: record.point,
            direction: scatter_direction,
        };
        *attenuation = self.albedo;
        true
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(r: i32, g: i32, b: i32, fuzz: f64) -> Self {
        Self {
            albedo: Color::rgb(r, g, b),
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }

    pub fn as_ref(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray_in.direction.reflect(record.normal).unit();
        *scattered = Ray {
            origin: record.point,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        };
        *attenuation = self.albedo;
        return scattered.direction.dot(record.normal) > 0.;
    }
}
