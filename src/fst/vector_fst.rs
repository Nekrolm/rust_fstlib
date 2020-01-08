use crate::fst::fst_traits;
use crate::fst::std_arc;
use fst_traits::ExpandedFst;
use fst_traits::Fst;
use fst_traits::Arc;
use std_arc::StdArc;
use std_arc::Weight;
use fst_traits::kNoStateId;

struct StateDescription {
    arcs : Vec<StdArc>,
    final_weight : Weight,
}


type StateId = <StdArc as Arc>::StateId;

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

impl Fst<StdArc> for VectorFst {
    type StateId = StateId;
    type Weight = Weight;
    type Arc = StdArc;

    fn Final(&self, state : Self::StateId) -> Self::Weight {
        return self.states[state as usize].final_weight;
    }

    fn Start(&self) -> Self::StateId {
        return self.start;
    }
}

impl ExpandedFst<StdArc> for VectorFst {
    fn NumStates(&self) -> Self::StateId {
        return self.states.len() as Self::StateId;
    }
    fn NumArcs(&self, state : Self::StateId) -> isize {
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

impl fst_traits::ArcIterator<StdArc, VectorFst> for ArcIterator<'_> {
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

impl fst_traits::StateIterator<StdArc, VectorFst> for StateIterator<'_> {

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


