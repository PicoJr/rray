use nalgebra::base::Scalar;
use nalgebra::{Point3, Vector3};

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
