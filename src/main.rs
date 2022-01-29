extern crate glam;
extern crate image;
extern crate rand;

use std::rc::Rc;

use glam::DVec3;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{thread_rng, Rng};
use rtweekend::{
    clamp, Camera, HitRecord, Hittable, HittableList, Lambertian, Material, Metal, Ray, Sphere,
};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> DVec3 {
    if depth <= 0 {
        return DVec3::ZERO;
    }

    let mut rec = HitRecord::EMPTY;
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(DVec3::ZERO, DVec3::ZERO);
        let mut attenuation = DVec3::ZERO;
        if rec
            .material
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return DVec3::ZERO;
        }
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

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: DVec3::new(0.8, 0.8, 0.0),
    });
    let material_center: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: DVec3::new(0.7, 0.3, 0.3),
    });
    let material_left: Rc<dyn Material> = Rc::new(Metal {
        albedo: DVec3::new(0.8, 0.8, 0.8),
    });
    let material_right: Rc<dyn Material> = Rc::new(Metal {
        albedo: DVec3::new(0.8, 0.6, 0.2),
    });

    let world = HittableList {
        objects: vec![
            Box::new(Sphere::new(
                DVec3::new(0.0, -100.5, -1.0),
                100.0,
                material_ground,
            )),
            Box::new(Sphere::new(
                DVec3::new(0.0, 0.0, -1.0),
                0.5,
                material_center,
            )),
            Box::new(Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.5, material_left)),
            Box::new(Sphere::new(DVec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
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
