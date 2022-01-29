use glam::DVec3;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

pub use camera::*;
pub use hittable::*;
pub use material::*;
pub use ray::*;
pub use sphere::*;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

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

pub fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = -uv.dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * n;
    return r_out_perp + r_out_parallel;
}
