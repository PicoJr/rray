use crate::ray::{Ray, RT};
use nalgebra::{Point3, Vector3};
use rand::prelude::{Distribution, ThreadRng};
use rand_distr::UnitDisc;

pub(crate) struct Camera {
    origin: Point3<RT>,
    lower_left_corner: Point3<RT>,
    horizontal: Vector3<RT>,
    vertical: Vector3<RT>,
    u: Vector3<RT>,
    v: Vector3<RT>,
    lens_radius: RT,
}

impl Camera {
    pub(crate) fn new(
        look_from: Point3<RT>,
        look_at: Point3<RT>,
        vup: Vector3<RT>,
        vfov: RT, // vertical field of view in degrees
        aspect_ratio: RT,
        aperture: RT,
        focus_dist: RT,
    ) -> Self {
        let theta = vfov / 180. as RT * std::f32::consts::PI as RT;
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (look_from - look_at).normalize();
        let u = (vup.cross(&w)).normalize();
        let v = w.cross(&u); // already normalized

        let origin = look_from;
        let horizontal = u.scale(viewport_width * focus_dist);
        let vertical = v.scale(viewport_height * focus_dist);
        let lower_left_corner =
            origin - horizontal.scale(0.5) - vertical.scale(0.5) - w.scale(focus_dist);

        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    pub(crate) fn get_ray(&self, s: RT, t: RT, thread_rng: &mut ThreadRng) -> Ray<RT> {
        let [dx_offset, dy_offset]: [RT; 2] = UnitDisc.sample(thread_rng);
        let offset =
            self.u.scale(dx_offset * self.lens_radius) + self.v.scale(dy_offset * self.lens_radius);
        let direction = self.lower_left_corner + self.horizontal.scale(s) + self.vertical.scale(t)
            - self.origin;
        Ray::new(self.origin + offset, direction - offset)
    }
}
