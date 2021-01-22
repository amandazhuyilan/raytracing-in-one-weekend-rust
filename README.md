# Raytracing in One Weekend, in Rust
Following the [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) tutorial in Rust.

At the core, a raytracer sends rays through pixels and computes the color seen in the directions of those rays. The involved steps are:
1. calculate the ray from eye to the pixel,
2. determine which ray the object intersects,
3. compute a color from that intersection point.

1. Sample Image Generator
The `sample_image_generator.rs` will generate a colorful `.ppm` image:

```bash
rustc sample_image_generator.rs
./sample_image_genrator > image.ppm
```
Then you can view the image with [ToyViewer](https://apps.apple.com/us/app/toyviewer/id414298354?l=en&mt=12) with the following command:
```bash
open -a ToyViewer image.ppm
```