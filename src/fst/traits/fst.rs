use super::{Arc, StateId};


pub trait BaseFst<ArcType :Arc, StateType : Copy + Clone = StateId> {
    fn Final(&self, state : StateType) -> ArcType::Weight;
    fn Start(&self) -> StateType;
}

pub trait ExpandedFst<ArcType :Arc, StateType : Copy + Clone = StateId> : BaseFst<ArcType, StateType> {
    fn NumStates(&self) -> StateType;
    fn NumArcs(&self, state : StateType) -> isize;
}

pub trait IterableFst<'a, ArcType : Arc,
                        StateType : Copy + Clone = StateId,
                        SelfType : BaseFst<ArcType, StateType> = Self>
: BaseFst<ArcType, StateType>  {
    type ArcIterator: ArcIterator <'a, ArcType, StateType, SelfType>;
    type StateIterator : StateIterator<'a, ArcType, StateType, SelfType>;
}

pub trait MutableFst<ArcType :Arc, StateType : Copy + Clone = StateId> :
    ExpandedFst<ArcType, StateType> {

    fn SetStart(&mut self, s: StateType);
    fn SetFinal(&mut self, s : StateType, w : ArcType::Weight);
    fn AddState(&mut self) -> StateType;
    fn AddArc(&mut self, s : StateType, arc : ArcType);
    fn DeleteStates(&mut self);
    fn DeleteArcs(&mut self, s : StateType);
}

pub trait StateIterator<'a, ArcType : Arc, StateType :  Copy + Clone, FST : BaseFst<ArcType, StateType>> {
    fn Value(&self) -> StateType;
    fn Done(&self) -> bool;
    fn Next(&mut self);

    fn new(fst : &'a FST) -> Self;
}

pub trait ArcIterator<'a, ArcType : Arc, StateType :  Copy + Clone, FST : BaseFst<ArcType, StateType>> {
    fn Value(&self) -> ArcType;
    fn Done(&self) -> bool;
    fn Next(&mut self);

    fn new(fst : &'a FST, s: StateType) -> Self;
}

