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

fn refract(uv: &Vector3<RT>, normal: &Vector3<RT>, etai_over_eta: RT) -> Vector3<RT> {
    let cos_theta = -uv.dot(normal);
    let r_out_perp = etai_over_eta * (uv + normal.scale(cos_theta));
    let r_out_parallel = normal.scale(-(1.0 as RT - r_out_perp.norm_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}

pub(crate) struct Dieletric {
    pub refraction_index: f64,
}

impl Scatterer for Dieletric {
    fn scatter(&self, ray: &Ray<f32>, ray_hit: &RayHit) -> Option<(RRgb, Ray<f32>)> {
        let attenuation = RRgb::new(1f64, 1f64, 1f64);
        let etai_over_etat = if ray_hit.front_face {
            1f64 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().normalize();
        let refracted = refract(&unit_direction, &ray_hit.normal, etai_over_etat as f32);
        Some((attenuation, Ray::new(ray_hit.point, refracted)))
    }
}
