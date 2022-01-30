use clap::Parser;
use glam::DVec3;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{thread_rng, Rng};
use std::rc::Rc;

use rtweekend::*;

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

fn random_scene() -> HittableList {
    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(DVec3::new(0.5, 0.5, 0.5)));

    let mut world = HittableList {
        objects: vec![Box::new(Sphere::new(
            DVec3::new(0.0, -1000.0, -1.0),
            1000.0,
            ground_material,
        ))],
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = thread_rng().gen_range(0.0..1.0);
            let center = DVec3::new(
                a as f64 + 0.9 * thread_rng().gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * thread_rng().gen_range(0.0..1.0),
            );

            if (center - DVec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    let color = DVec3::new(
                        thread_rng().gen_range(0.0..1.0),
                        thread_rng().gen_range(0.0..1.0),
                        thread_rng().gen_range(0.0..1.0),
                    );
                    let albedo = color * color;
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = DVec3::new(
                        thread_rng().gen_range(0.5..1.0),
                        thread_rng().gen_range(0.5..1.0),
                        thread_rng().gen_range(0.5..1.0),
                    );
                    let fuzz = thread_rng().gen_range(0.0..0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    Rc::new(Dielectric::new(1.5))
                };
                world
                    .objects
                    .push(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    world.objects.push(Box::new(Sphere::new(
        DVec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));

    world.objects.push(Box::new(Sphere::new(
        DVec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(DVec3::new(0.4, 0.2, 0.1))),
    )));

    world.objects.push(Box::new(Sphere::new(
        DVec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(DVec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 1200)]
    width: u32,

    #[clap(short, long, default_value_t = 800)]
    height: u32,

    #[clap(short, long, default_value_t = 500)]
    samples_per_pixel: u32,

    #[clap(short, long, default_value_t = 50)]
    max_depth: u32,

    #[clap(short, long, default_value = "scene.png")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let aspect_ratio = args.width as f64 / args.height as f64;

    let mut rng = thread_rng();

    let world = random_scene();

    let lookfrom = DVec3::new(13.0, 3.0, 2.0);
    let lookat = DVec3::new(0.0, 0.0, 0.0);
    let vup = DVec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut img: RgbImage = ImageBuffer::new(args.width, args.height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color = DVec3::ZERO;
        for _s in 0..args.samples_per_pixel {
            let x = x as f64;
            let y = (args.height - y) as f64;

            let u = (x + rng.gen_range(0.0..1.0)) / (args.width as f64 - 1.0);
            let v = (y + rng.gen_range(0.0..1.0)) / (args.height as f64 - 1.0);

            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(&ray, &world, args.max_depth);
        }
        *pixel = rgb_color(&pixel_color, args.samples_per_pixel);
    }

    img.save(args.output).unwrap();
}
