use crate::math;

pub fn write_color(pixel_color: math::Vec3) {
    // in the textbook it the color_* fields are implemented as doubles,
    // but the ppm takes ints and rounds discards the decimal part,
    // messing up the blue->white gradient picture.
    let color_x = (255.999 * pixel_color.x) as i32;
    let color_y = (255.999 * pixel_color.y) as i32;
    let color_z = (255.999 * pixel_color.z) as i32;
    println!("{} {} {}", color_x, color_y, color_z);
}
