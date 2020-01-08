

pub type StateId = i32;
pub type Label = i32;

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
    type Weight;
}

impl<Weight> Arc for ArcTpl<Weight> {
    type Weight = Weight;
}


pub trait Fst<'a, ArcType : Arc> {
    type Arc;

    type ArcIterator : ArcIterator<ArcType>;
    type StateIterator : StateIterator;

    fn Final(&self, state : StateId) -> <ArcType as Arc>::Weight;
    fn Start(&self) -> StateId;

    fn MakeArcIterator(&'a self, state : StateId) -> Self::ArcIterator;
    fn MakeStateIterator(&'a self) -> Self::StateIterator;
}

pub trait ExpandedFst<'a, ArcType : Arc> : Fst<'a, ArcType> {
    fn NumStates(&self) -> StateId;
    fn NumArcs(&self, state : StateId) -> isize;
}

pub trait StateIterator {
    fn Value(&self) -> StateId;
    fn Done(&self) -> bool;
    fn Next(&mut self);
}

pub trait ArcIterator<ArcType : Arc> {
    fn Value(&self) -> ArcType;
    fn Done(&self) -> bool;
    fn Next(&mut self);
}
