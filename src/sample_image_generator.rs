fn main() {
    // Image
    const IMAGE_WIDTH: i16 = 256;
    const IMAGE_HEIGHT: i16 = 256;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT-1).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = ((i as f64) / (IMAGE_WIDTH-1) as f64) as f64;
            let g = ((j as f64) / (IMAGE_HEIGHT-1) as f64) as f64;
            let b = 0.25 as f64;

            let ir = (255.999 * r) as i16;
            let ig = (255.999 * g) as i16;
            let ib = (255.999 * b) as i16;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}