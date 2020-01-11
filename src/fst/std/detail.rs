use super::weight;

use super::super::generic;
use super::super::traits;

pub type Arc = generic::Arc<weight::Weight>;
pub type VectorFst = generic::VectorFst<Arc>;
pub type ConstFst = generic::ConstFst<Arc>;

pub trait Fst<'a, SelfType : traits::BaseFst<Arc> = Self> :
    traits::IterableFst<'a, Arc, traits::StateId, SelfType> {}

impl<'a> Fst<'a> for VectorFst {}
impl<'a> Fst<'a> for ConstFst {}