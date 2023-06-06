fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("255");

    for y in 0..image_height{
        for x in 0..image_width{
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
