use super::super::traits;
use super::super::generic;

use std::f32::INFINITY;

impl traits::Semiring for f32 {
    fn Plus(lhs : Self, rhs : Self) -> Self {
        return lhs.min(rhs);
    }
    fn Times(lhs : Self, rhs : Self) -> Self {
        return lhs + rhs;
    }

    fn Zero() -> Self {
        return INFINITY;
    }
    fn One() -> Self {
        return 0.;
    }
}

pub type Weight = generic::Weight<f32>;
