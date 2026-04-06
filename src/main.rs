mod vec3;
mod color;
fn main() {
    //Image we want to create
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    //Render Image
    println!("P3\n{image_width} {image_height} \n255");

    for i in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", image_height-i);
        for j in 0..image_width {
            let pixel_color = crate::color::Color::new(i as f64 / (image_width - 1) as f64,
                                         j as f64 / (image_height - 1) as f64,
                                         0.0);
            crate::color::write_color(pixel_color);
        }
    }
    eprintln!("\rDone                                          ");
}
