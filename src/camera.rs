use crate::{degrees_to_radians, Ray};
use glam::DVec3;

pub struct Camera {
    pub origin: DVec3,
    pub lower_left_corner: DVec3,
    pub horizontal: DVec3,
    pub vertical: DVec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = DVec3::ZERO;
        let horizontal = DVec3::new(viewport_width, 0.0, 0.0);
        let vertical = DVec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        );
    }
}
