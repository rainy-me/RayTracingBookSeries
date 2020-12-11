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

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn as_ref(self) -> Arc<Self> {
        Arc::new(self)
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1. - refraction_index) / (1. + refraction_index);
        r0 = r0 * r0;
        return r0 + (1. - r0) * (1. - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if record.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.unit();

        let cos_theta = (-unit_direction).dot(record.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
        {
            unit_direction.reflect(record.normal)
        } else {
            unit_direction.refract(record.normal, refraction_ratio)
        };

        *scattered = Ray {
            origin: record.point,
            direction,
        };
        return true;
    }
}
