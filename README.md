# RRay

![example](res/example_512.png)

Implementation of the Peter Shirley's book: [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust.

Requires rust nightly `#![feature(total_cmp)]`.

## Example

Compile in release mode.

```
cargo +nightly build --release
```

Run (generates `out.png`).

```
./target/release/rray -w 512 -m 20 -s 20
```
