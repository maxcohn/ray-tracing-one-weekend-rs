

mod vec3;
use vec3::{Vec3, Color, Point3};

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scan lines left: {}", j);
        for i in 0..IMAGE_WIDTH {
            let vec = Color::from(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.25
            );
            vec.print_color();
        }
    }

    

}
