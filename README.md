# Raytracing in One Weekend, in Rust
Following the [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) tutorial in Rust.

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