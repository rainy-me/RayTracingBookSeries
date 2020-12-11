use crate::*;
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
    pub fn new() -> Self {
        Lambertian {
            albedo: Vec3::default(),
        }
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
            direction: reflected,
        };
        *attenuation = self.albedo;
        return scattered.direction.dot(record.normal) > 0.;
    }
}
