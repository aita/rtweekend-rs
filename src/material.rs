use crate::near_zero;
use crate::random_unit_vector;
use crate::reflect;
use crate::HitRecord;
use crate::Ray;
use glam::DVec3;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut DVec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: DVec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut DVec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: DVec3,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut DVec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(ray.direction().normalize(), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(rec.normal) > 0.0
    }
}
