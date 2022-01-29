use crate::{degrees_to_radians, Ray};
use glam::DVec3;

pub struct Camera {
    pub origin: DVec3,
    pub lower_left_corner: DVec3,
    pub horizontal: DVec3,
    pub vertical: DVec3,
}

impl Camera {
    pub fn new(lookfrom: DVec3, lookat: DVec3, vup: DVec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        );
    }
}
