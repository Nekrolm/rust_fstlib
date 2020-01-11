use super::super::traits;

#[derive(Copy, Clone)]
pub struct Weight {
    value : f32
}

use std::f32::INFINITY;


impl traits::Weight for Weight {
    type ValueType = f32;

    fn new(value : Self::ValueType) -> Self {
        Weight{value}
    }

    fn Value(&self) -> Self::ValueType {
        return self.value;
    }
    fn SetValue(&mut self, v : Self::ValueType) {
        self.value = v;
    }

    fn Plus(lhs : Self, rhs : Self) -> Self {
        return Weight::new(lhs.value.min(rhs.value));
    }
    fn Times(lhs : Self, rhs : Self) -> Self {
        return Weight::new(lhs.value + rhs.value)
    }


    fn Zero() -> Self {
        return Weight::new(INFINITY);
    }
    fn One() -> Self {
        return Weight::new(0.);
    }
}
