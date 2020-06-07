use std::f64;

use rand::Rng;

mod camera;
mod hittable;
mod ray;
mod util;
mod vec3;
mod material;

use camera::*;
use hittable::*;
use ray::Ray;
use vec3::{Color, Point3, Vec3};
use material::Material;

//TODO: once finished, randomly generate sphere to place around


//TODO: make these adjustable - cli args
const SAMPLES_PER_PIXEL: usize = 100;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 384;
const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
const MAX_DEPTH: u32 = 50;

/// Get the color of the ray so that we can get a blue to white gradient
fn ray_color<T: Hittable>(ray: &Ray, world: &T, depth: u32) -> Color {
    let mut rec = HitRecord::new();


    if depth <= 0 {
        return Color::new();
    }

    // Check if the given object is going to be hit by the given ray
    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Color::new();

        let material = rec.material;

        if material.scatter(ray, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::new();
    }

    let unit_dir: Vec3 = ray.direction().unit_vector();

    let t = 0.5 * (unit_dir.y() + 1.0);

    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    let samples_per_pixel: u32;//: usize = 100;
    let image_width: u32;//: usize = 384;
    let image_height: u32;//: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;

    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 0 {
        samples_per_pixel = args[0].parse().unwrap();
        image_width = args[1].parse().unwrap();
    } else {
        samples_per_pixel = 100;
        image_width = 384;   
    }
    image_height = ((image_width as f64) / ASPECT_RATIO) as u32;



    // file header
    println!("P3"); // specifies that colors are in ASCII
    println!("{} {}", image_width, image_height); // columns and rows
    println!("255"); // max color

    let mut rng = rand::thread_rng();

    // General steps for race tracing:
    // 1. Calculate ray from eye to pixel
    // 2. Determine which objects the ray intersects
    // 3. Compute a color for that intersection point

    let cam = Camera::new();

    let mut world = HittableList::new();

    world.push(Box::new(Sphere::from(
        Point3::from(0.0,0.0,-1.0), 0.5, Material::Lambertian { albedo: Color::from(0.7, 0.3, 0.3)}
    )));

    world.push(Box::new(Sphere::from(
        Point3::from(0.0,-100.5,-1.0), 100.0, Material::Lambertian { albedo: Color::from(0.8, 0.8, 0.0)}
    )));

    world.push(Box::new(Sphere::from(
        Point3::from(1.0,0.0,-1.0), 0.5, Material::Metal { albedo: Color::from(0.8, 0.6, 0.2), fuzz: 0.3 }
    )));

    world.push(Box::new(Sphere::from(
        Point3::from(-1.0,0.0,-1.0), 0.5, Material::Lambertian { albedo: Color::from(0.8, 0.8, 0.8)}
    )));



    for j in (0..image_height).rev() {
        eprintln!("Scan lines left: {}", j);
        for i in 0..image_width {
            let mut color = Color::new();

            // Sampling loop. We take a bunch of sample with slight shifts in
            // location (within 1.0 units from the current position), add them
            // together, and then take an average. This creates a smoother look
            // on edges
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64) as f64;
                let v = ((j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64) as f64;

                let ray = cam.get_ray(u, v);
                color += ray_color(&ray, &world, MAX_DEPTH);
            }

            color.print_color(samples_per_pixel);
        }
    }
}
