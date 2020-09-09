use crate::color::RRgb;
use crate::ray::{random_in_unit_sphere, Ray, RayHit, RT};
use nalgebra::Vector3;

pub(crate) trait Scatterer {
    /// returns color attenuation and scattered ray
    fn scatter(&self, ray: &Ray<RT>, ray_hit: &RayHit) -> Option<(RRgb, Ray<RT>)>;
}

pub(crate) struct Lambertian {
    pub albedo: RRgb,
}

impl Scatterer for Lambertian {
    fn scatter(&self, _ray: &Ray<f32>, ray_hit: &RayHit) -> Option<(RRgb, Ray<f32>)> {
        let scatter_direction = ray_hit.normal + random_in_unit_sphere(&mut rand::thread_rng());
        let scattered = Ray::new(ray_hit.point, scatter_direction);
        Some((self.albedo.clone(), scattered))
    }
}

pub(crate) struct Metal {
    pub albedo: RRgb,
}

fn reflect(v: &Vector3<RT>, normal: &Vector3<RT>) -> Vector3<RT> {
    v - normal.scale((2. as RT) * v.dot(normal))
}

impl Scatterer for Metal {
    fn scatter(&self, ray: &Ray<f32>, ray_hit: &RayHit) -> Option<(RRgb, Ray<f32>)> {
        let reflected = reflect(&ray.direction().normalize(), &ray_hit.normal);
        let scattered = Ray::new(ray_hit.point, reflected);
        if scattered.direction().dot(&ray_hit.normal) > (0. as RT) {
            Some((self.albedo.clone(), scattered))
        } else {
            None
        }
    }
}
