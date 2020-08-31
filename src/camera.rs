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
        origin: Point3<RT>,
        aspect_ratio: RT,
        viewport_height: RT,
        focal_length: RT,
    ) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = Vector3::new(viewport_width, 0., 0.);
        let vertical = Vector3::new(0., viewport_height, 0.);
        let lower_left_corner = origin
            - horizontal.scale(0.5)
            - vertical.scale(0.5)
            - Vector3::new(0., 0., focal_length);
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
