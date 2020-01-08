use crate::fst::fst_traits;
use crate::fst::std_arc;
use fst_traits::ExpandedFst;
use fst_traits::Fst;
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

impl<'a> Fst<'a, StdArc> for ConstFst {
    type Arc = StdArc;

    type ArcIterator = ArcIterator<'a>;
    type StateIterator = StateIterator<'a>;

    fn Final(&self, state : StateId) -> <Self::Arc as fst_traits::Arc>::Weight {
        return self.states[state as usize].final_weight;
    }
    fn Start(&self) -> StateId {
        return self.start;
    }

    fn MakeArcIterator(&'a self, state : StateId) -> Self::ArcIterator {
        return Self::ArcIterator::new(self, state);
    }
    fn MakeStateIterator(&'a self) -> Self::StateIterator {
        return Self::StateIterator::new(self);
    }
}

impl ExpandedFst<'_, StdArc> for ConstFst {
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

impl<'a> ArcIterator<'a> {
    pub fn new (fst: &'a ConstFst, state : StateId) -> Self {
        let arc_range = fst.states[state as usize].arcs;
        return ArcIterator{ vec : arc_range, fst, pos : arc_range.begin };
    }
}

impl fst_traits::ArcIterator<StdArc> for ArcIterator<'_> {
    fn Value(&self) -> StdArc {
        return self.fst.arcs[self.pos]
    }

    fn Done(&self) -> bool {
        return self.pos == self.vec.end;
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
    pub fn new (fst: &'a ConstFst) -> Self {
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


impl ConstFst {
    pub  fn new<'a, FST : Fst<'a,StdArc>>(fst : &'a FST) -> ConstFst {
        let mut result = ConstFst {
            start : fst_traits::kNoStateId,
            arcs : Vec::new(),
            states : Vec::new(),
        };


        let mut states = & mut result.states;


        use fst_traits::ArcIterator;
        use fst_traits::StateIterator;

        {
            let mut siter = fst.MakeStateIterator();
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
            let mut siter = fst.MakeStateIterator();
            while !siter.Done() {
                let state = siter.Value() as usize;
                let arcs_begin = result.arcs.len();
                let mut aiter = fst.MakeArcIterator(siter.Value());
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