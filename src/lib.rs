pub use camera::Camera;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use ray::Ray;
pub use sphere::Sphere;

mod camera;
mod hittable;
mod ray;
mod sphere;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}
