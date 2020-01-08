use crate::fst::fst_traits;

use fst_traits::ArcTpl;
use std::f32::INFINITY;


#[derive(Copy, Clone)]
pub struct Weight {
    value : f32
}

impl Weight {
    pub fn new(val : f32) -> Weight {
        return Weight{value : val}
    }

    pub fn Zero() -> Weight {
        return Weight{value:INFINITY}
    }
    pub fn One() -> Weight {
        return Weight{value:0.}
    }


    pub fn Value(&self) -> f32 {
        return self.value;
    }
}

pub type StdArc = ArcTpl<Weight>;


