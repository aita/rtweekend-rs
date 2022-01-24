extern crate glam;
extern crate image;
extern crate rand;

use glam::DVec3;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{thread_rng, Rng};
use rtweekend::{clamp, Camera, HitRecord, Hittable, HittableList, Ray, Sphere};

fn random_in_unit_sphere() -> DVec3 {
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

fn random_in_hemisphere(normal: DVec3) -> DVec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> DVec3 {
    if depth <= 0 {
        return DVec3::ZERO;
    }

    let mut rec = HitRecord::EMPTY;

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + random_in_hemisphere(rec.normal);
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * DVec3::ONE + t * DVec3::new(0.5, 0.7, 1.0)
}

fn rgb_color(pixel_color: &DVec3, samples_per_pixel: u32) -> Rgb<u8> {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let scale = 1.0 / (samples_per_pixel as f64);
    let r = (r * scale).sqrt();
    let g = (g * scale).sqrt();
    let b = (b * scale).sqrt();

    let r = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let g = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let b = (256.0 * clamp(b, 0.0, 0.999)) as u8;
    Rgb([r, g, b])
}

fn main() {
    let mut rng = thread_rng();

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let world = HittableList {
        objects: vec![
            Box::new(Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };

    let camera = Camera::new();

    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color = DVec3::ZERO;
        for _s in 0..SAMPLES_PER_PIXEL {
            let x = x as f64;
            let y = (HEIGHT - y) as f64;

            let u = (x + rng.gen_range(0.0..1.0)) / (WIDTH as f64 - 1.0);
            let v = (y + rng.gen_range(0.0..1.0)) / (HEIGHT as f64 - 1.0);

            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(&ray, &world, MAX_DEPTH);
        }
        *pixel = rgb_color(&pixel_color, SAMPLES_PER_PIXEL);
    }

    img.save("scene.png").unwrap();
}
