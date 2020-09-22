use crate::ray::{Ray, RT};
use nalgebra::{Point3, Vector3};

pub(crate) struct Camera {
    origin: Point3<RT>,
    lower_left_corner: Point3<RT>,
    horizontal: Vector3<RT>,
    vertical: Vector3<RT>,
}

impl Camera {
    pub(crate) fn new(
        look_from: Point3<RT>,
        look_at: Point3<RT>,
        vup: Vector3<RT>,
        vfov: RT, // vertical field of view in degrees
        aspect_ratio: RT,
    ) -> Self {
        let theta = vfov / 180. as RT * std::f32::consts::PI as RT;
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (look_from - look_at).normalize();
        let u = (vup.cross(&w)).normalize();
        let v = w.cross(&u); // already normalized

        let origin = look_from;
        let horizontal = u.scale(viewport_width);
        let vertical = v.scale(viewport_height);
        let lower_left_corner = origin - horizontal.scale(0.5) - vertical.scale(0.5) - w;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub(crate) fn get_ray(&self, u: RT, v: RT) -> Ray<RT> {
        let direction = self.lower_left_corner + self.horizontal.scale(u) + self.vertical.scale(v)
            - self.origin;
        Ray::new(self.origin, direction)
    }
}
