extern crate image;

mod ray;

type CT = u8;

use image::{ImageBuffer, Rgb};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use nalgebra::{Point3, Vector3};
use rayon::prelude::*;

use crate::ray::{Ray, RT};

fn ray_color(ray: &Ray<RT>) -> image::Rgb<CT> {
    let sphere_center = Point3::new(0., 0., -1.);
    if let Some(intersection_t) = hit_sphere(&sphere_center, 0.5, ray) {
        let unormal = (ray.at(intersection_t) - sphere_center).normalize();
        let r = 0.5 * (255. * (unormal.x + 1.0));
        let g = 0.5 * (255. * (unormal.y + 1.0));
        let b = 0.5 * (255. * (unormal.z + 1.0));
        image::Rgb([r as u8, g as u8, b as u8])
    } else {
        let t = 0.5 * (ray.direction().normalize().y + 1.0);
        let s = (u8::max_value() as RT * t) as u8;
        image::Rgb([s, s, s])
    }
}

fn hit_sphere(center: &Point3<RT>, radius: RT, ray: &Ray<RT>) -> Option<RT> {
    let oc: Vector3<RT> = ray.origin() - center;
    let a = ray.direction().norm_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.norm_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        None // no intersection
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}

fn main() -> anyhow::Result<()> {
    // image
    let aspect_ratio: RT = 2.0 / 1.0;
    let image_width: u32 = 128;
    let image_height = (image_width as RT / aspect_ratio) as u32;
    assert!(image_width > 0 && image_height > 0);

    // camera
    let viewport_height: RT = 2.0;
    let viewport_width: RT = aspect_ratio * viewport_height;
    let focal_length: RT = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vector3::new(viewport_width, 0., 0.);
    let vertical = Vector3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal.scale(0.5) - vertical.scale(0.5) - Vector3::new(0., 0., focal_length);

    let progress_bar = ProgressBar::new((image_width * image_height) as u64)
        .with_style(ProgressStyle::default_bar().template("{bar} [{elapsed}] ETA {eta}"));
    let pixels: Vec<(u32, u32, Rgb<u8>)> = (0..(image_width * image_height))
        .into_par_iter()
        .progress_with(progress_bar)
        .map(|p| (p as u32 % image_width, p as u32 / image_width))
        .map(|(x, y)| {
            let u = x as RT / image_width as RT;
            let v = y as RT / image_height as RT;
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal.scale(u) + vertical.scale(v) - origin,
            );
            let color = ray_color(&ray);
            (x, y, color)
        })
        .collect();
    let mut img = ImageBuffer::new(image_width as u32, image_height as u32);
    for (x, y, pixel) in pixels {
        img.put_pixel(x, y, pixel);
    }
    img.save("out.png")?;
    Ok(())
}
