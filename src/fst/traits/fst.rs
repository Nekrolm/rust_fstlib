use super::{Arc, StateId};


pub trait BaseFst<ArcType :Arc, StateType = StateId> {
    fn Final(&self, state : StateType) -> ArcType::Weight;
    fn Start(&self) -> StateType;
}

pub trait ExpandedFst<ArcType :Arc, StateType = StateId> : BaseFst<ArcType, StateType> {
    fn NumStates(&self) -> StateType;
    fn NumArcs(&self, state : StateType) -> isize;
}

pub trait IterableFst<'a, ArcType : Arc, StateType = StateId, SelfType : BaseFst<ArcType, StateType> = Self>
: BaseFst<ArcType, StateType>  {
    type ArcIterator: ArcIterator <'a, ArcType, StateType, SelfType>;
    type StateIterator : StateIterator<'a, ArcType, StateType, SelfType>;
}

pub trait StateIterator<'a, ArcType : Arc, StateType, FST : BaseFst<ArcType, StateType>> {
    fn Value(&self) -> StateType;
    fn Done(&self) -> bool;
    fn Next(&mut self);

    fn new(fst : &'a FST) -> Self;
}

pub trait ArcIterator<'a, ArcType : Arc, StateType, FST : BaseFst<ArcType, StateType>> {
    fn Value(&self) -> ArcType;
    fn Done(&self) -> bool;
    fn Next(&mut self);

    fn new(fst : &'a FST, s: StateType) -> Self;
}

