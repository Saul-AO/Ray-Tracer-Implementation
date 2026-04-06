fn main() {
    //Image we want to create
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    //Render Image
    println!("P3\n{image_width} {image_height} \n255");

    for i in 0..image_height {
        for j in 0..image_width {
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64;
            let b: f64 = 0.0;

            let ir: usize = (255.999 * r) as usize;
            let ig: usize = (255.999 * g) as usize;
            let ib: usize = (255.999 * b) as usize;

            println!("{ir} {ig} {ib}");
        }
    }
}
