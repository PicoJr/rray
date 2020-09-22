#![feature(total_cmp)]
extern crate image;
#[macro_use]
extern crate clap;

mod camera;
mod cli;
mod color;
mod material;
mod ray;

use image::{ImageBuffer, Rgb};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use nalgebra::{Point3, Vector3};
use rayon::prelude::*;

use crate::camera::Camera;
use crate::color::RRgb;
use crate::material::{Dieletric, Lambertian, Metal, Scatterer};
use crate::ray::{shoot_ray, Hittable, Ray, Sphere, RT};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::sync::Arc;

fn ray_color(ray: &Ray<RT>, hittables: &[Arc<dyn Hittable + Send + Sync>], depth: usize) -> RRgb {
    if depth == 0 {
        return RRgb::new(0., 0., 0.);
    }
    let hit = shoot_ray(hittables, ray, 0.01, RT::INFINITY);
    match hit {
        Some(ray_hit) => {
            if let Some((attenuation, scattered)) = ray_hit.material.scatter(ray, &ray_hit) {
                attenuation * ray_color(&scattered, hittables, depth - 1)
            } else {
                RRgb::new(0., 0., 0.) // no ray scattered
            }
        }
        None => {
            let t = 0.5 * (ray.direction().normalize().y + 1.0); // t in [0;1[
            let s = u8::max_value() as f64 * t as f64;
            RRgb::new(s, s, s)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let app = cli::get_app();
    let matches = app.get_matches();
    let config = cli::RConfig::from_matches(matches)?;

    // image
    let aspect_ratio: RT = 16.0 / 9.0;
    let image_width: u32 = config.image_width as u32;
    let image_height = (image_width as RT / aspect_ratio) as u32;
    assert!(image_width > 0 && image_height > 0);
    let sample_per_pixel = config.sample_per_pixel;
    let max_depth = config.max_depth;

    // camera
    let look_from = Point3::new(0., 5., 5.);
    let look_at = Point3::new(0., 0., -1.);
    let vup = Vector3::new(0., 1., 0.);
    let vfov = 90.;
    let camera = Camera::new(look_from, look_at, vup, vfov, aspect_ratio);

    let material_ground = Arc::new(Lambertian {
        albedo: RRgb::new(0.8, 0.8, 0.),
    });
    let material_metal = Arc::new(Metal {
        albedo: RRgb::new(0.8, 0.8, 0.8),
    });
    let material_dieletric = Arc::new(Dieletric {
        refraction_index: 1.5f64,
    });
    let material_lambertian = Arc::new(Lambertian {
        albedo: RRgb::new(0.8, 0.6, 0.2),
    });

    let ground = Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    let mut world: Vec<Arc<dyn Hittable + Send + Sync>> = vec![ground];
    let mut rng = thread_rng();
    let side = Uniform::new(0., 1.);
    for dx in -10..=10 {
        for dz in -10..=0 {
            let rdm = rng.sample(side);
            let material: Arc<dyn Scatterer + Send + Sync> = if rdm < 0.80 {
                let r = rng.sample(side);
                let g = rng.sample(side);
                let b = rng.sample(side);
                Arc::new(Lambertian {
                    albedo: RRgb::new(r, g, b),
                })
            } else if rdm < 0.90 {
                material_metal.clone()
            } else {
                material_dieletric.clone()
            };
            world.push(Arc::new(Sphere::new(
                Point3::new(0.0 + dx as RT, 0.0, 0.0 + dz as RT),
                (rdm * rdm) as RT,
                material,
            )))
        }
    }

    let primary_rays = image_width * image_height; // 1 ray / pixel

    let progress_bar = ProgressBar::new(primary_rays as u64)
        .with_style(ProgressStyle::default_bar().template("{bar} [{elapsed}] ETA {eta}"));
    progress_bar.set_draw_delta((primary_rays / 1000) as u64); // limit progress_bar redraw
    let pixels: Vec<(u32, u32, Rgb<u8>)> = (0..primary_rays)
        .into_par_iter()
        .progress_with(progress_bar)
        .map(|p| (p as u32 % image_width, p as u32 / image_width))
        .map(|(x, y)| {
            let mut rng = thread_rng();
            let side = Uniform::new(0., 1.);
            let sum_color: RRgb = (0..sample_per_pixel)
                .map(|_| {
                    let du = rng.sample(side);
                    let dv = rng.sample(side);
                    let u = (x as RT + du as RT) / image_width as RT;
                    let v = (y as RT + dv as RT) / image_height as RT;
                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, &world, max_depth)
                })
                .sum();
            let average_color = sum_color * (1. / (sample_per_pixel as RT));
            (x, y, average_color.into())
        })
        .collect();
    let mut img = ImageBuffer::new(image_width as u32, image_height as u32);
    for (x, y, pixel) in pixels {
        let inverted_y = image_height - y - 1; // invert y axis, our raytracer camera y axis points upward, the image crate points downward
        img.put_pixel(x, inverted_y, pixel);
    }
    img.save(config.output_file_path)?;
    Ok(())
}
