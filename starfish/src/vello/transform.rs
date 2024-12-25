use std::ops::{Mul, MulAssign};

use crate::interface::render_backend::Transform as TTransform;

use vello::kurbo::Affine;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform(pub(crate) Affine);

impl From<Affine> for Transform {
    fn from(transform: Affine) -> Self {
        Transform(transform)
    }
}

impl Mul<Self> for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Transform(self.0 * rhs.0)
    }
}

impl MulAssign for Transform {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl TTransform for Transform {

}
