

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

pub trait BaseFst<ArcType : Arc> {
    type Arc;

    fn Final(&self, state : StateId) -> <ArcType as Arc>::Weight;
    fn Start(&self) -> StateId;
}

pub trait ExpandedFst<ArcType : Arc> : BaseFst<ArcType> {
    fn NumStates(&self) -> StateId;
    fn NumArcs(&self, state : StateId) -> isize;
}

pub trait IterableFst<'a, ArcType : Arc, SelfType : BaseFst<ArcType> = Self> : BaseFst<ArcType>  {
    type ArcIterator: ArcIterator <'a, ArcType, SelfType>;
    type StateIterator : StateIterator<'a, ArcType, SelfType>;
}

pub trait StateIterator<'a, ArcType : Arc, FST : BaseFst<ArcType>> {
    fn Value(&self) -> StateId;
    fn Done(&self) -> bool;
    fn Next(&mut self);

    fn new(fst : &'a FST) -> Self;
}

pub trait ArcIterator<'a, ArcType : Arc, FST : BaseFst<ArcType>> {
    fn Value(&self) -> ArcType;
    fn Done(&self) -> bool;
    fn Next(&mut self);

    fn new(fst : &'a FST, s: StateId) -> Self;
}

pub fn MakeArcIterator<'a, ArcType : Arc, FST : IterableFst<'a, ArcType>>(fst : &'a FST, state : StateId) -> FST::ArcIterator {
    return FST::ArcIterator::new(fst, state);
}

pub fn MakeStateIterator<'a, ArcType : Arc, FST : IterableFst<'a, ArcType>>(fst : &'a FST) -> FST::StateIterator {
    return FST::StateIterator::new(fst);
}