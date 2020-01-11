use super::super::traits;


pub struct ArcIterator {}

pub struct StateIterator {}

impl ArcIterator {
    pub fn new<'a, ArcType : traits::Arc,
                StateType,
                FST : traits::IterableFst<'a, ArcType, StateType>>(fst : &'a FST,
                                                        state : StateType)
        -> FST::ArcIterator {
        use traits::ArcIterator;
        return FST::ArcIterator::new(fst, state);
    }
}

impl StateIterator {
    pub fn new<'a, ArcType : traits::Arc, StateType,
                FST : traits::IterableFst<'a, ArcType, StateType>>(fst : &'a FST)
        -> FST::StateIterator {
        use traits::StateIterator;
        return FST::StateIterator::new(fst);
    }
}


