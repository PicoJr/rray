extern crate image;

use image::ImageBuffer;
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> anyhow::Result<()> {
    let (width, height) = (256, 128);
    assert!(width > 0 && height > 0);
    let progress_bar = ProgressBar::new(width * height)
        .with_style(ProgressStyle::default_bar().template("{bar} [{elapsed}] ETA {eta}"));
    let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let r = ((x as f64 / width as f64) * 255.) as u8;
        let g = ((y as f64 / height as f64) * 255.) as u8;
        let b = 100;
        progress_bar.inc(1);
        image::Rgb([r, g, b])
    });
    progress_bar.finish();
    img.save("out.png")?;
    Ok(())
}
