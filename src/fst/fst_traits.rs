

type StateId = i32;
type Label = i32;

pub const kNoStateId : StateId = -1;
pub const kNoLabel : Label = -1;

#[derive(Copy, Clone)]
pub struct ArcTpl<Weight> {
    pub ilabel : Label,
    pub olabel : Label,
    pub weight : Weight,
    pub nextstate : StateId
}


pub trait Arc {
    type StateId;
    type Label;
    type Weight;
}

impl<Weight> Arc for ArcTpl<Weight> {
    type StateId = StateId;
    type Label = Label;
    type Weight = Weight;
}


pub trait Fst<ArcType : Arc> {
    type StateId;
    type Weight;
    type Arc;

    fn Final(&self, state : Self::StateId) -> Self::Weight;
    fn Start(&self) -> Self::StateId;
}

pub trait ExpandedFst<ArcType : Arc> : Fst<ArcType> {
    fn NumStates(&self) -> Self::StateId;
    fn NumArcs(&self, state : Self::StateId) -> isize;
}

pub trait StateIterator<ArcType : Arc, FstType : Fst<ArcType>> {
    fn Value(&self) -> FstType::StateId;
    fn Done(&self) -> bool;
    fn Next(&mut self);
}

pub trait ArcIterator<ArcType : Arc, FstType : Fst<ArcType>> {
    fn Value(&self) -> FstType::Arc;
    fn Done(&self) -> bool;
    fn Next(&mut self);
}
