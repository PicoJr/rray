use nalgebra::base::Scalar;
use nalgebra::{Point3, Vector3};
use std::cmp::Ordering;
use std::sync::Arc;

pub(crate) type RT = f32;

#[derive(Clone, Debug)]
pub(crate) struct Ray<T: Scalar> {
    origin: Point3<T>,
    direction: Vector3<T>,
}

impl Ray<RT> {
    pub(crate) fn new(origin: Point3<RT>, direction: Vector3<RT>) -> Self {
        Ray { origin, direction }
    }

    pub(crate) fn at(&self, t: RT) -> Point3<RT> {
        self.origin + self.direction.scale(t)
    }

    pub(crate) fn origin(&self) -> Point3<RT> {
        self.origin
    }
    pub(crate) fn direction(&self) -> Vector3<RT> {
        self.direction
    }
}

pub(crate) struct RayHit {
    /// where the ray hit
    pub point: Point3<RT>,
    /// normalized normal
    pub normal: Vector3<RT>,
    /// when the ray hit
    pub t: RT,
}

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray<RT>, t_min: RT, t_max: RT) -> Option<RayHit>;
}

pub(crate) struct Sphere {
    center: Point3<RT>,
    radius: RT,
}

impl Sphere {
    pub fn new(center: Point3<RT>, radius: RT) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray<f32>, t_min: f32, t_max: f32) -> Option<RayHit> {
        let oc: Vector3<RT> = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t1 = (-half_b - root) / a;
            let t2 = (-half_b + root) / a;
            let t = if t_min < t1 && t1 < t_max {
                Some(t1)
            } else if t_min < t2 && t2 < t_max {
                Some(t2)
            } else {
                None
            };
            match t {
                Some(t) => {
                    let point = ray.at(t);
                    let normal = (point - self.center).scale(1. / self.radius);
                    Some(RayHit {
                        point,
                        normal,
                        t: t1,
                    })
                }
                None => None,
            }
        } else {
            None
        }
    }
}

pub(crate) fn shoot_ray(
    hittables: &[Arc<dyn Hittable + Send + Sync>],
    ray: &Ray<RT>,
    t_min: RT,
    t_max: RT,
) -> Option<RayHit> {
    let closest_hit =
        hittables
            .iter()
            .map(|g| g.hit(ray, t_min, t_max))
            .min_by(
                |hit_maybe, other_hit_maybe| match (hit_maybe, other_hit_maybe) {
                    (None, None) => Ordering::Equal,
                    (Some(_h), None) => Ordering::Less,
                    (None, Some(_h)) => Ordering::Greater,
                    (Some(h), Some(other)) => h.t.total_cmp(&other.t),
                },
            );
    match closest_hit {
        Some(maybe_hit) => maybe_hit,
        _ => None,
    }
}
