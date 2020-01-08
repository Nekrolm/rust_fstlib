use crate::fst::fst_traits;
use crate::fst::std_arc;
use fst_traits::ExpandedFst;
use fst_traits::Fst;
use fst_traits::Arc;
use fst_traits::StateId;
use std_arc::StdArc;
use std_arc::Weight;
use fst_traits::kNoStateId;

struct StateDescription {
    arcs : Vec<StdArc>,
    final_weight : Weight,
}



pub struct VectorFst {
    start : StateId,
    states : Vec<StateDescription>
}


impl VectorFst {
    pub fn new() -> VectorFst {
        return VectorFst{start : kNoStateId, states : Vec::new()};
    }

    pub fn AddState(&mut self) -> StateId {
        let new_state_id = self.states.len();
        let new_state = StateDescription{arcs : Vec::new(), final_weight : Weight::Zero() };
        self.states.push(new_state);
        return new_state_id as StateId;
    }

    pub fn AddArc(&mut self, state : StateId, arc : StdArc) {
        self.states[state as usize].arcs.push(arc);
    }

    pub fn SetFinal(&mut self, state : StateId, w : Weight) {
        self.states[state as usize].final_weight = w;
    }

    pub fn SetStart(&mut self, state : StateId) {
        self.start = state;
    }
}

impl<'a> Fst<'a, StdArc> for VectorFst {
    type Arc = StdArc;
//
    type ArcIterator = ArcIterator<'a>;
    type StateIterator = StateIterator<'a>;


    fn Final(&self, state : StateId) -> <Self::Arc as Arc>::Weight {
        return self.states[state as usize].final_weight;
    }

    fn Start(&self) -> StateId {
        return self.start;
    }

    fn MakeArcIterator(&'a self, state : StateId) -> Self::ArcIterator {
        return ArcIterator::new(self, state);
    }
    fn MakeStateIterator(&'a self) -> Self::StateIterator {
        return StateIterator::new(self);
    }
}

impl ExpandedFst<'_, StdArc> for VectorFst {
    fn NumStates(&self) -> StateId {
        return self.states.len() as StateId;
    }
    fn NumArcs(&self, state : StateId) -> isize {
        return self.states[state as usize].arcs.len() as isize;
    }
}

pub struct ArcIterator<'a> {
    vec : &'a Vec<StdArc>,
    pos : usize
}

impl<'a> ArcIterator<'a> {
    pub fn new (fst: &'a VectorFst, state : StateId) -> Self {
        return ArcIterator{ vec : &fst.states[state as usize].arcs, pos : 0 };
    }
}

impl fst_traits::ArcIterator<StdArc> for ArcIterator<'_> {
    fn Value(&self) -> StdArc {
        return self.vec[self.pos]
    }

    fn Done(&self) -> bool {
        return self.pos == self.vec.len();
    }

    fn Next(&mut self) {
        self.pos += 1;
    }
}

pub struct StateIterator<'a> {
    vec : &'a Vec<StateDescription>,
    pos : usize
}

impl <'a> StateIterator<'a> {
    pub fn new (fst: &'a VectorFst) -> Self {
        return StateIterator{ vec : &fst.states, pos : 0 };
    }
}

impl fst_traits::StateIterator for StateIterator<'_> {

    fn Value(&self) -> StateId {
        return self.pos as StateId;
    }

    fn Done(&self) -> bool {
        return self.pos == self.vec.len();
    }

    fn Next(&mut self) {
        self.pos += 1;
    }
}


