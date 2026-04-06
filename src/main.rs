mod color;
mod ray;
mod vec3;
use crate::color::{Color, write_color};
use std::io::{self, Write};
//Return type either returns void, or I/O Error
fn main() -> io::Result<()> {
    //Image we want to create
    let image_width: i32 = 256;
    let image_height: i32 = 256;
    //Create a lock beforehand to just print our data and unlock at the end
    let mut stdout = io::stdout().lock();
    //The '?' tells the compiler to try this. If it fails, crash. Else, keep going
    writeln!(stdout, "P3\n{image_width} {image_height}\n255")?;

    for j in 0..image_height {
        //Included a progress reporter in order to indicate the output
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush()?; // Flush to ensure it displays immediately

        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );

            // Pass the locked stdout stream to our write_color function
            write_color(&mut stdout, pixel_color)?;
        }
    }
    eprintln!("\rDone.                 ");
    //Equivalent of saying 'return 0' in C++
    Ok(())
}
