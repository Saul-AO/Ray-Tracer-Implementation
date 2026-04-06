//Color Header File

pub struct Color {
    r : f64,
    g : f64,
    b : f64
}

impl Color {
    pub fn new(r : f64, g : f64, b : f64) -> Color {
        Color { r, g, b}
    }
}

pub fn write_color(pixel_color : Color) {
    let rbyte = (255.999 * pixel_color.r) as usize;
    let gbyte = (255.999 * pixel_color.g) as usize;
    let bbyte = (255.999 * pixel_color.b) as usize;
    println!("{rbyte} {gbyte} {bbyte}");
}