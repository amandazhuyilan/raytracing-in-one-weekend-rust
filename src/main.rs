mod color;
mod ray;
mod math;

use math::Vec3 as Color;
use math::Vec3 as Point3;

fn hit_sphere(center: Point3, radius: f64, r: &ray::Ray) -> f64 {
    let oc = r.origin - center;
    let a = math::vec3::get_dot_product(r.direction, r.direction);
    let b = 2.0 * math::vec3::get_dot_product(oc, r.direction);
    let c = math::vec3::get_dot_product(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt() ) / (2.0 * a);
    }
}

fn ray_color(r: ray::Ray) -> Color {
    let t = hit_sphere(Point3{x: 0.0, y: 0.0, z: -1.0}, 0.5, &r);
    if t > 0.0 {
        // Rendering surface normals on a sphere
        let n = math::vec3::get_unit_vector(r.at(t) - math::Vec3{x: 0.0, y: 0.0, z: -1.0});
        return 0.5 * Color{x: n.x + 1.0, y: n.y + 1.0, z: n.z + 1.0}
    }
    // color red any pixel that hits a small sphere we place at −1 on the z-axis
    if hit_sphere(Point3{x: 0.0, y: 0.0, z: -1.0}, 0.5, &r) > 0.0{
        return Color {x: 1.0, y: 0.0, z: 0.0};
    }
    let unit_direction = math::vec3::get_unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color{x: 1.0, y: 1.0, z: 1.0} + t * Color{x: 0.5, y: 0.7, z: 1.0};
}

fn main() {
    // The following code writes a sky blue -> white gradient image.
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as i32;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0 as f64;
    let viewport_width = aspect_ratio * viewport_height as f64;
    let focal_length = 1.0 as f64;

    let origin = math::Vec3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = math::Vec3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = math::Vec3{x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - math::Vec3{x:0.0, y:0.0, z:focal_length};

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height-1).rev() {
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let r = ray::Ray{origin: origin, direction: lower_left_corner + u * horizontal + v * vertical - origin};
            let pixel_color = ray_color(r);
            color::write_color(pixel_color);
            // assert!(false);
        }
    }
}
