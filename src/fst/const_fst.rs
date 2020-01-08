use crate::fst::fst_traits;
use crate::fst::std_arc;
use fst_traits::ExpandedFst;
use fst_traits::Fst;
use fst_traits::Arc;
use std_arc::StdArc;

struct ArcRange {
    begin : isize,
    end : isize
}

struct StateDescription {
    arcs : ArcRange,
    final_weight : <StdArc as Arc>::Weight,
}


pub struct ConstFst {
    start : <StdArc as Arc>::StateId,
    arcs : Vec<StdArc>,
    states : Vec<StateDescription>,
}

impl Fst<StdArc> for ConstFst {

    type StateId = <StdArc as Arc>::StateId;
    type Weight = <StdArc as Arc>::Weight;
    type Arc = StdArc;

    fn Final(&self, state : Self::StateId) ->Self::Weight {
        return self.states[state as usize].final_weight;
    }
    fn Start(&self) -> Self::StateId {
        return self.start;
    }
}

impl ExpandedFst<StdArc> for ConstFst {
    fn NumStates(&self) -> Self::StateId {
        return self.states.len() as Self::StateId;
    }
    fn NumArcs(&self, state : Self::StateId) -> isize {
        let state_descr = &self.states[state as usize];
        let arc_indexer = &state_descr.arcs;
        return arc_indexer.end - arc_indexer.begin;
    }
}