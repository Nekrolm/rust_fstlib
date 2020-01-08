use crate::fst::fst_traits;

use fst_traits::Arc;
use std::f32::INFINITY;

pub type Label = i32;
pub type StateId = i32;


pub const kNoStateId : StateId = -1;

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

#[derive(Copy, Clone)]
pub struct StdArc {
    pub ilabel : Label,
    pub olabel : Label,
    pub weight : Weight,
    pub nextstate : StateId
}

impl Arc for StdArc {
    type StateId = StateId;
    type Label = Label;
    type Weight = Weight;
}

