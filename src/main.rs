mod color;
mod ray;
mod vec3;
use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, unit_vector};
use std::io::{self, Write};

pub fn ray_color(r: Ray) -> Color {
    let unit_direction: Vec3 = unit_vector(r.direction());
    let a: f64 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color { e: [1.0, 1.0, 1.0] } + a * Color { e: [0.5, 0.7, 1.0] }
}
//Return type either returns void, or I/O Error
fn main() -> io::Result<()> {
    //Image we want to create
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;

    //Calculate the image height, and ensure it is at least 1
    let mut image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    //Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3 { e: [0.0, 0.0, 0.0] };

    //Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3 {
        e: [viewport_width, 0.0, 0.0],
    };
    let viewport_v = Vec3 {
        e: [0.0, -viewport_height, 0.0],
    };

    //Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

    //Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Vec3 {
            e: [0.0, 0.0, focal_length],
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    //Create a lock beforehand to just print our data and unlock at the end
    let mut stdout = io::stdout().lock();
    //The '?' tells the compiler to try this. If it fails, crash. Else, keep going
    writeln!(stdout, "P3\n{image_width} {image_height}\n255")?;

    for j in 0..image_height {
        //Included a progress reporter in order to indicate the output
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush()?; // Flush to ensure it displays immediately

        for i in 0..image_width {
            // 1. Find the exact center of the current pixel
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            // 2. Determine the direction of the ray from the camera to the pixel
            let ray_direction = pixel_center - camera_center;

            // 3. Create the Ray
            // Use whatever instantiation matches your ray.rs file. E.g.:
            let r = Ray::new(camera_center, ray_direction);
            // 4. Determine the color using the ray
            let pixel_color = ray_color(r);
            // Pass the locked stdout stream to our write_color function
            write_color(&mut stdout, pixel_color)?;
        }
    }
    eprintln!("\rDone.                 ");
    //Equivalent of saying 'return 0' in C++
    Ok(())
}
