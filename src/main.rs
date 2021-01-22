mod color;

fn main() {
    // Image
    const IMAGE_WIDTH: i16 = 256;
    const IMAGE_HEIGHT: i16 = 256;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT-1).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = color::vec3::Vec3{
                x: (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                y: (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                z: 0.25,
            };
        color::write_color(pixel_color);
        }
    }
}
