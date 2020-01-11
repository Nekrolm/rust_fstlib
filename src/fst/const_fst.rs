use crate::fst::fst_traits;
use crate::fst::std_arc;
use fst_traits::ExpandedFst;
use fst_traits::{IterableFst, BaseFst};
use fst_traits::Arc;
use std_arc::StdArc;
use std_arc::Weight;
use fst_traits::StateId;

#[derive(Copy, Clone)]
struct ArcRange {
    begin : usize,
    end : usize
}

#[derive(Copy, Clone)]
struct StateDescription {
    arcs : ArcRange,
    final_weight : <StdArc as Arc>::Weight,
}

impl StateDescription {
    fn new() -> StateDescription {
        return StateDescription{ arcs : ArcRange{begin :0,
                                                end:0},
                                final_weight : Weight::Zero() }
    }
}


pub struct ConstFst {
    start : StateId,
    arcs : Vec<StdArc>,
    states : Vec<StateDescription>,
}

impl BaseFst<StdArc> for ConstFst {
    type Arc = StdArc;

    fn Final(&self, state : StateId) -> <Self::Arc as fst_traits::Arc>::Weight {
        return self.states[state as usize].final_weight;
    }
    fn Start(&self) -> StateId {
        return self.start;
    }
}

impl ExpandedFst<StdArc> for ConstFst {
    fn NumStates(&self) -> StateId {
        return self.states.len() as StateId;
    }
    fn NumArcs(&self, state : StateId) -> isize {
        let state_descr = &self.states[state as usize];
        let arc_indexer = &state_descr.arcs;
        return (arc_indexer.end - arc_indexer.begin) as isize;
    }
}

pub struct ArcIterator<'a> {
    vec : ArcRange,
    fst : &'a ConstFst,
    pos : usize
}

pub struct StateIterator<'a> {
    vec : &'a Vec<StateDescription>,
    pos : usize
}

impl<'a> IterableFst<'a, StdArc> for ConstFst {
    type ArcIterator = ArcIterator<'a>;
    type StateIterator = StateIterator<'a>;
}

impl<'a> fst_traits::ArcIterator<'a, StdArc, ConstFst> for ArcIterator<'a> {
    fn Value(&self) -> StdArc {
        return self.fst.arcs[self.pos]
    }

    fn Done(&self) -> bool {
        return self.pos == self.vec.end;
    }

    fn Next(&mut self) {
        self.pos += 1;
    }

    fn new (fst: &'a ConstFst, state : StateId) -> Self {
        let arc_range = fst.states[state as usize].arcs;
        return ArcIterator{ vec : arc_range, fst, pos : arc_range.begin };
    }
}


impl<'a> fst_traits::StateIterator<'a, StdArc, ConstFst> for StateIterator<'a> {

    fn Value(&self) -> StateId {
        return self.pos as StateId;
    }

    fn Done(&self) -> bool {
        return self.pos == self.vec.len();
    }

    fn Next(&mut self) {
        self.pos += 1;
    }

    fn new (fst: &'a ConstFst) -> Self {
        return StateIterator{ vec : &fst.states, pos : 0 };
    }

}


impl ConstFst {
    pub  fn new<'a, FST : IterableFst<'a,StdArc>>(fst : &'a FST) -> ConstFst {
        let mut result = ConstFst {
            start : fst_traits::kNoStateId,
            arcs : Vec::new(),
            states : Vec::new(),
        };


        let mut states = & mut result.states;


        use fst_traits::MakeArcIterator;
        use fst_traits::MakeStateIterator;

        use fst_traits::ArcIterator;
        use fst_traits::StateIterator;

        {
            let mut siter = FST::StateIterator::new(fst);
            while !siter.Done() {
                let state = siter.Value();
                if state as usize >= states.len() {
                    states.resize(state as usize + 1, StateDescription::new());
                }
                siter.Next();
            }
            states.shrink_to_fit();
        }

        {
            let mut siter = MakeStateIterator(fst);
            while !siter.Done() {
                let state = siter.Value() as usize;
                let arcs_begin = result.arcs.len();
                let mut aiter = MakeArcIterator(fst,siter.Value());
                while !aiter.Done() {
                    result.arcs.push(aiter.Value());
                    aiter.Next();
                }
                let arcs_end = result.arcs.len();
                let arcs = ArcRange{begin : arcs_begin, end : arcs_end};
                let weight = fst.Final(siter.Value() );
                states[state] = StateDescription{arcs, final_weight : weight as Weight};
                siter.Next();
            }
        }
        result.start = fst.Start();
        result.arcs.shrink_to_fit();
        return result;
    }
}