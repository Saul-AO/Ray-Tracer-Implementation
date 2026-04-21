mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::rtweekend::INFINITY;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3, unit_vector};

use std::io::{self, Write};
use std::rc::Rc;

//We use the ray_color in order for the ray to:
//1. Calculate the ray from the 'eye' through the pixel
//2. Determine which objects the ray intersects
//3. Compute a color for the closest intersection point
// * Ray now includes a variable 'world' of type array of Hittables
pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    // 1. Check if the ray hits anything in the entire world
    // We use 0.0 for t_min and INFINITY for t_max
    if let Some(rec) = world.hit(r, 0.0, INFINITY) {
        // If it hits, calculate the color based on the surface normal
        return 0.5
            * Color::new(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            );
    }
    // 2. If it hit nothing, draw the background gradient
    let unit_direction: Vec3 = unit_vector(r.direction());
    let a: f64 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
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
    // World
    let mut world = HittableList::new();

    // Add the center sphere
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    // Add a massive sphere below it to act as the ground
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
            //Find the exact center of the current pixel
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            //Determine the direction of the ray from the camera to the pixel
            let ray_direction = pixel_center - camera_center;

            //Create the Ray using 'camera_center' as origin and 'ray_direction' as direction
            let r = Ray::new(camera_center, ray_direction);
            //Determine the color using the ray AND world
            let pixel_color = ray_color(&r, &world);
            // Pass the locked stdout stream to our write_color function
            write_color(&mut stdout, pixel_color)?;
        }
    }
    eprintln!("\rDone.                 ");
    //Equivalent of saying 'return 0' in C++
    Ok(())
}
