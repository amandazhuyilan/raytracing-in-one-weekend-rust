use crate::math;

pub fn write_color(pixel_color: math::Vec3) {
    let color_x = (255.999 * pixel_color.x) as f64;
    let color_y = (255.999 * pixel_color.y) as f64;
    let color_z = (255.999 * pixel_color.z) as f64;
    println!("{} {} {}", color_x, color_y, color_z);
}

