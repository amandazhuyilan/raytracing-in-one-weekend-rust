mod color;
mod ray;
mod math;

type Color = math::Vec3;
type Ray = math::Vec3;

// fn ray_color(const ray& r) {
//     vec3 unit_direction = unit_vector(r.direction());
//     auto t = 0.5*(unit_direction.y() + 1.0);
//     return (1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
// }

// fn ray_color()

fn main() {
    // Image
    const IMAGE_WIDTH: i16 = 256;
    const IMAGE_HEIGHT: i16 = 256;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT-1).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = math::Vec3{
                x: (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                y: (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                z: 0.25,
            };
        color::write_color(pixel_color);
        }
    }
}
