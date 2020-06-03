

mod vec3;
mod ray;
use ray::Ray;
use vec3::{Vec3, Color, Point3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 384;
const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;


/// Calculate whether or not we hit the sphere located at -1 on the z-axis
///
/// t^2 b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0
fn hit_spehere(center: Point3, radius: f64, ray: &Ray) -> bool {
    /*
    let oc = ray.origin() - center; // A-C
    let a = ray.direction().dot(ray.direction()); // b⋅b
    let b = 2.0 * oc.dot(ray.direction()); // 2b⋅(A−C)
    let c = oc.dot(oc) - radius * radius; // (A−C)⋅(A−C)−r^2
    let discriminant = b * b - 4.0 * a * c;

    // we have not found everything we can, with t being our only unknown

    // if the discriminant is greater than zero, that means our ray hits the sphere
    // at least once
    discriminant > 0.0
    */

    let oc = ray.origin() - center;
    
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    
    if discriminant > 0.0 {
        eprintln!("{:?}", ray.direction());
        eprintln!("{:?}", ray.direction().dot(ray.direction()));
        //eprintln!("{:?}", oc);
    }
    return (discriminant > 0.0);
}

/// Get the color of the ray so that we can get a blue to white gradient
fn ray_color(ray: &Ray) -> Color {
    if hit_spehere(Point3::from(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::from(1.0, 0.0, 0.0);
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



    // General steps for race tracing:
    // 1. Calculate ray from eye to pixel
    // 2. Determine which objects the ray intersects
    // 3. Compute a color for that intersection point

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);


    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scan lines left: {}", j);
        for i in 0..IMAGE_WIDTH {

            let u = (i as f64 / (IMAGE_WIDTH - 1) as f64) as f64;
            let v = (j as f64 / (IMAGE_HEIGHT - 1) as f64) as f64;

            let ray = Ray::from(origin, lower_left_corner + u * horizontal +  v * vertical  - origin);

            let color = ray_color(&ray);
            color.print_color();
        }
    }

    

}
