//Color Header File
use crate::vec3::Vec3;
use std::io::Write;

//Type alias for Color
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) -> std::io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    //Translate the [0,1] component values to byte range
    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    //Write out the pixels for result return
    writeln!(out, "{rbyte} {gbyte} {bbyte}")
}
