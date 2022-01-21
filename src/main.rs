extern crate glam;
extern crate image;

use glam::DVec3;
use image::{ImageBuffer, Rgb, RgbImage};
use tinyraytracer::{HitRecord, Hittable, HittableList, Ray, Sphere};

fn ray_color(r: &Ray, world: &dyn Hittable) -> DVec3 {
    let mut rec = HitRecord::EMPTY;
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + DVec3::ONE);
    }
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * DVec3::ONE + t * DVec3::new(0.5, 0.7, 1.0)
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

    let world = HittableList {
        objects: vec![
            Box::new(Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = DVec3::ZERO;
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

        let pixel_color = ray_color(&ray, &world);
        *pixel = rgb_color(&pixel_color);
    }

    img.save("scene.png").unwrap();
}
