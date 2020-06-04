use std::f64;

use rand::Rng;

mod camera;
mod hittable;
mod ray;
mod util;
mod vec3;

use camera::*;
use hittable::*;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

const SAMPLES_PER_PIXEL: usize = 100;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 384;
const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;

/// Get the color of the ray so that we can get a blue to white gradient
fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
    }

    let unit_dir: Vec3 = ray.direction().unit_vector();

    let t = 0.5 * (unit_dir.y() + 1.0);

    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    // file header
    println!("P3"); // specifies that colors are in ASCII
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT); // columns and rows
    println!("255"); // max color

    let mut rng = rand::thread_rng();

    // General steps for race tracing:
    // 1. Calculate ray from eye to pixel
    // 2. Determine which objects the ray intersects
    // 3. Compute a color for that intersection point

    let cam = Camera::new();

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
    )));

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scan lines left: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new();

            // Sampling loop. We take a bunch of sample with slight shifts in
            // location (within 1.0 units from the current position), add them
            // together, and then take an average. This creates a smoother look
            // on edges
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64) as f64;
                let v = ((j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64) as f64;

                let ray = cam.get_ray(u, v);
                color += ray_color(&ray, &world);
            }

            color.print_color(SAMPLES_PER_PIXEL);
        }
    }
}
