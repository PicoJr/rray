use crate::material::Material;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use nalgebra::base::Scalar;
use nalgebra::{Point3, Vector3};
use rand::prelude::ThreadRng;
use rand_distr::{Distribution, UnitBall};
use std::cmp::Ordering;

pub(crate) type RT = f32;

fn bvh_position(p: Point3<RT>) -> bvh::nalgebra::Point3<RT> {
    bvh::nalgebra::Point3::new(p.x, p.y, p.z)
}

fn bvh_direction(v: Vector3<RT>) -> bvh::nalgebra::Vector3<RT> {
    bvh::nalgebra::Vector3::new(v.x, v.y, v.z)
}

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
    pub material: Material,
    /// when the ray hit
    pub t: RT,
    pub front_face: bool,
}

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray<RT>, t_min: RT, t_max: RT) -> Option<RayHit>;
}

pub(crate) struct Sphere {
    center: Point3<RT>,
    radius: RT,
    material: Material,
    node_index: usize, // bvh node index, must be unique
}

impl Sphere {
    pub fn new(center: Point3<RT>, radius: RT, material: Material, node_index: usize) -> Self {
        Sphere {
            center,
            radius,
            material,
            node_index,
        }
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
                    let outward_normal = (point - self.center).scale(1. / self.radius);
                    let front_face = ray.direction.dot(&outward_normal) < 0f32;
                    let normal = if front_face {
                        outward_normal // front hit
                    } else {
                        -outward_normal
                    };
                    Some(RayHit {
                        point,
                        normal,
                        material: self.material.clone(),
                        t,
                        front_face,
                    })
                }
                None => None,
            }
        } else {
            None
        }
    }
}

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        let half_size = Vector3::new(self.radius, self.radius, self.radius);
        let min = bvh_position(self.center - half_size);
        let max = bvh_position(self.center + half_size);
        AABB::with_bounds(min, max)
    }
}

impl BHShape for Sphere {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

pub(crate) enum Target {
    Sphere(Sphere),
}

impl Hittable for Target {
    fn hit(&self, ray: &Ray<f32>, t_min: f32, t_max: f32) -> Option<RayHit> {
        match self {
            Target::Sphere(s) => s.hit(ray, t_min, t_max),
        }
    }
}

impl Bounded for Target {
    fn aabb(&self) -> AABB {
        match self {
            Target::Sphere(s) => s.aabb(),
        }
    }
}

impl BHShape for Target {
    fn set_bh_node_index(&mut self, index: usize) {
        match self {
            Target::Sphere(s) => s.set_bh_node_index(index),
        }
    }

    fn bh_node_index(&self) -> usize {
        match self {
            Target::Sphere(s) => s.bh_node_index(),
        }
    }
}

pub(crate) fn shoot_ray(
    ray: &Ray<RT>,
    world: &[Target],
    bvh: &BVH,
    t_min: RT,
    t_max: RT,
) -> Option<RayHit> {
    let bvh_ray = bvh::ray::Ray::new(bvh_position(ray.origin()), bvh_direction(ray.direction()));
    let aabb_hits = bvh.traverse(&bvh_ray, world);

    let closest_hit =
        aabb_hits
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

pub(crate) fn random_in_unit_sphere(thread_rng: &mut ThreadRng) -> Vector3<RT> {
    let v: [RT; 3] = UnitBall.sample(thread_rng);
    Vector3::new(v[0], v[1], v[2])
}
