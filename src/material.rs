use crate::color::RRgb;
use crate::ray::{random_in_unit_sphere, Ray, RayHit, RT};
use nalgebra::Vector3;
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Clone)]
pub(crate) enum Material {
    Dieletric(Dieletric),
    Lambertian(Lambertian),
    Metal(Metal),
    Light(Light),
}

impl Scatterer for Material {
    fn scatter(
        &self,
        ray: &Ray<f32>,
        ray_hit: &RayHit,
        thread_rng: &mut ThreadRng,
    ) -> Option<(RRgb, Ray<f32>)> {
        match self {
            Material::Dieletric(dieletric) => dieletric.scatter(ray, ray_hit, thread_rng),
            Material::Lambertian(lambertian) => lambertian.scatter(ray, ray_hit, thread_rng),
            Material::Metal(metal) => metal.scatter(ray, ray_hit, thread_rng),
            Material::Light(_) => None, // does not scatter light
        }
    }
}

impl Emitter for Material {
    fn emit(&self) -> RRgb {
        match self {
            Material::Dieletric(_) => RRgb::new(0., 0., 0.),
            Material::Lambertian(_) => RRgb::new(0., 0., 0.),
            Material::Metal(_) => RRgb::new(0., 0., 0.),
            Material::Light(light) => light.emit(),
        }
    }
}

pub(crate) trait Scatterer {
    /// returns color attenuation and scattered ray
    fn scatter(
        &self,
        ray: &Ray<RT>,
        ray_hit: &RayHit,
        thread_rng: &mut ThreadRng,
    ) -> Option<(RRgb, Ray<RT>)>;
}

pub(crate) trait Emitter {
    fn emit(&self) -> RRgb;
}

#[derive(Clone)]
pub(crate) struct Light {
    pub emitted: RRgb,
}

impl Emitter for Light {
    fn emit(&self) -> RRgb {
        self.emitted.clone()
    }
}

#[derive(Clone)]
pub(crate) struct Lambertian {
    pub albedo: RRgb,
}

impl Scatterer for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray<f32>,
        ray_hit: &RayHit,
        thread_rng: &mut ThreadRng,
    ) -> Option<(RRgb, Ray<f32>)> {
        let scatter_direction = ray_hit.normal + random_in_unit_sphere(thread_rng);
        let scattered = Ray::new(ray_hit.point, scatter_direction);
        Some((self.albedo.clone(), scattered))
    }
}

#[derive(Clone)]
pub(crate) struct Metal {
    pub albedo: RRgb,
}

fn reflect(v: &Vector3<RT>, normal: &Vector3<RT>) -> Vector3<RT> {
    v - normal.scale((2. as RT) * v.dot(normal))
}

impl Scatterer for Metal {
    fn scatter(
        &self,
        ray: &Ray<f32>,
        ray_hit: &RayHit,
        _thread_rng: &mut ThreadRng,
    ) -> Option<(RRgb, Ray<f32>)> {
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

#[derive(Clone)]
pub(crate) struct Dieletric {
    pub refraction_index: f64,
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1f64 - refraction_index) / (1f64 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1f64 - r0) * (1f64 - cosine).powf(5f64)
}

impl Scatterer for Dieletric {
    fn scatter(
        &self,
        ray: &Ray<f32>,
        ray_hit: &RayHit,
        thread_rng: &mut ThreadRng,
    ) -> Option<(RRgb, Ray<f32>)> {
        let attenuation = RRgb::new(1f64, 1f64, 1f64);
        let etai_over_etat = if ray_hit.front_face {
            1f64 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().normalize();

        let cos_theta = f64::min(-unit_direction.dot(&ray_hit.normal) as f64, 1f64);
        let sin_theta = (1f64 - cos_theta * cos_theta).sqrt();

        let reflected_probability = schlick(cos_theta, etai_over_etat);
        let side = Uniform::new(0., 1.);
        let randomly_reflected = thread_rng.sample(side) < reflected_probability;

        let scattered = if randomly_reflected || etai_over_etat * sin_theta > 1f64 {
            // reflected
            reflect(&unit_direction, &ray_hit.normal)
        } else {
            // refracted
            refract(&unit_direction, &ray_hit.normal, etai_over_etat as f32)
        };

        Some((attenuation, Ray::new(ray_hit.point, scattered)))
    }
}
