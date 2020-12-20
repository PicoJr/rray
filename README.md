# RRay

![example](res/example_512.png)

Implementation of the Peter Shirley's book: [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust.

## Example

Compile in release mode.

```
cargo build --release
```

Run (generates `out.png`).

```
./target/release/rray -w 512 -m 50 -s 1000 --vfov 60 --aperture 0.1 --parallel -o out.png
```
