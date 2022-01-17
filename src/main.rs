extern crate glam;
extern crate image;

use glam::DVec3;
use image::{ImageBuffer, Rgb, RgbImage};

mod ray;
use ray::Ray;

fn hit_sphere(center: DVec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> DVec3 {
    let t = hit_sphere(DVec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - DVec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * (n + 1.0);
    }
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * DVec3::new(1.0, 1.0, 1.0) + t * DVec3::new(0.5, 0.7, 1.0)
}

fn rgb_color(pixel_color: &DVec3) -> Rgb<u8> {
    let r = (255.999 * pixel_color.x) as u8;
    let g = (255.999 * pixel_color.y) as u8;
    let b = (255.999 * pixel_color.z) as u8;
    Rgb([r, g, b])
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECT_RATIO) as u32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = DVec3::new(0.0, 0.0, 0.0);
    let horizontal = DVec3::new(viewport_width, 0.0, 0.0);
    let vertical = DVec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, focal_length);

    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = (x as f64) / (WIDTH as f64 - 1.0);
        let v = ((HEIGHT - y) as f64) / (HEIGHT as f64 - 1.0);
        let ray = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );

        let pixel_color = ray_color(&ray);
        *pixel = rgb_color(&pixel_color);
    }

    img.save("scene.png").unwrap();
}
