crate::vec3;

pub fn write_color(Vec3: pixel_color) {
    let color_x = (255.999 * pixel_color.x) as f64;
    let color_y = (255.999 * pixel_color.y) as f64;
    let color_z = (255.999 * pixel_color.z) as f64;
    println!("{} {} {}", color_x, color_y, color_z);
}
