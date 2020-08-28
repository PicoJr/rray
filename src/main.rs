extern crate image;

use image::{ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let (width, height): (u32, u32) = (512, 256);
    assert!(width > 0 && height > 0);
    let progress_bar = ProgressBar::new((width * height) as u64)
        .with_style(ProgressStyle::default_bar().template("{bar} [{elapsed}] ETA {eta}"));
    let pixels: Vec<(u32, u32, Rgb<u8>)> = (0..(width * height))
        .into_par_iter()
        .progress_with(progress_bar)
        .map(|p| (p as u32 % width, p as u32 / width))
        .map(|(x, y)| {
            let r = ((x as f64 / width as f64) * 255.) as u8;
            let g = ((y as f64 / height as f64) * 255.) as u8;
            let b = 100;
            (x, y, image::Rgb([r, g, b]))
        })
        .collect();
    let mut img = ImageBuffer::new(width as u32, height as u32);
    for (x, y, pixel) in pixels {
        img.put_pixel(x, y, pixel);
    }
    img.save("out.png")?;
    Ok(())
}
