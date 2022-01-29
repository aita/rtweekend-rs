pub use camera::Camera;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use material::{Lambertian, Material, Metal};
pub use ray::Ray;
pub use sphere::Sphere;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;

use glam::DVec3;
use rand::{thread_rng, Rng};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}

pub fn near_zero(e: &DVec3) -> bool {
    // Return true if the vector is close to zero in all dimensions.
    let s = 1e-8;
    e[0].abs() < s && e[1].abs() < s && e[2].abs() < s
}

pub fn random_in_unit_sphere() -> DVec3 {
    loop {
        let p = DVec3::new(
            thread_rng().gen_range(-1.0..1.0),
            thread_rng().gen_range(-1.0..1.0),
            thread_rng().gen_range(-1.0..1.0),
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> DVec3 {
    random_in_unit_sphere().normalize()
}

pub fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    return v - 2.0 * v.dot(n) * n;
}
