mod weight;
mod binary_io;
pub use weight::*;
pub use binary_io::*;

use super::generic;
use super::traits;

pub type Arc = generic::Arc<weight::Weight>;
pub type VectorFst = generic::VectorFst<Arc>;
pub type ConstFst = generic::ConstFst<Arc>;

pub trait Fst<'a, SelfType : traits::BaseFst<Arc> = Self> :
traits::IterableFst<'a, Arc, traits::StateId, SelfType> {}
pub trait MutableFst : traits::MutableFst<Arc> {}

impl<'a> Fst<'a> for VectorFst {}
impl<'a> Fst<'a> for ConstFst {}
impl MutableFst for VectorFst {}

// forward generic make functions for iterators
pub use generic::ArcIterator;
pub use generic::StateIterator;

//forward traits
pub use traits::ArcIterator as ArcIteratorTrait;
pub use traits::StateIterator as StateIteratorTrait;
pub use traits::Weight as WeightTraits;
pub use traits::Arc as ArcTraits;