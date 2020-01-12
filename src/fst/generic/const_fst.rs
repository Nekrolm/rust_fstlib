use super::super::traits;
use super::super::generic;
use crate::fst::traits::StateId;

#[derive(Copy, Clone)]
struct ArcRange {
    begin : usize,
    end : usize
}

#[derive(Copy, Clone)]
struct StateDescription<ArcType : traits::Arc> {
    arcs : ArcRange,
    final_weight : ArcType::Weight,
}

impl<ArcType : traits::Arc> StateDescription<ArcType> {
    fn new() -> Self {
        use traits::Weight;
        return StateDescription{ arcs : ArcRange{begin :0,
                                                end:0},
                                final_weight : ArcType::Weight::Zero() }
    }
}


pub struct ConstFst<ArcType : traits::Arc> {
    start : traits::StateId,
    arcs : Vec<ArcType>,
    states : Vec<StateDescription<ArcType>>,
}

impl<ArcType : traits::Arc> traits::BaseFst<ArcType> for ConstFst<ArcType> {
    fn Final(&self, state : StateId) -> ArcType::Weight {
        return self.Final(state);
    }
    fn Start(&self) -> StateId {
        return self.Start();
    }
}

impl<ArcType : traits::Arc> traits::ExpandedFst<ArcType> for ConstFst<ArcType> {
    fn NumStates(&self) -> StateId {
        return self.NumStates();
    }
    fn NumArcs(&self, state : StateId) -> isize {
        return self.NumArcs(state);
    }
}

mod details {
    use super::traits;
    use super::ConstFst;
    use super::StateId;

    pub struct ArcIterator<'a, ArcType: traits::Arc> {
        vec: super::ArcRange,
        fst: &'a ConstFst<ArcType>,
        pos: usize
    }

    pub struct StateIterator<'a, ArcType: traits::Arc> {
        vec: &'a Vec<super::StateDescription<ArcType>>,
        pos: usize
    }


    impl<'a, ArcType: traits::Arc> traits::ArcIterator<'a, ArcType, StateId, ConstFst<ArcType>>
    for ArcIterator<'a, ArcType> {
        fn Value(&self) -> ArcType {
            return self.fst.arcs[self.pos]
        }

        fn Done(&self) -> bool {
            return self.pos == self.vec.end;
        }

        fn Next(&mut self) {
            self.pos += 1;
        }

        fn new(fst: &'a ConstFst<ArcType>,
               state: StateId) -> Self {
            let arc_range = fst.states[state as usize].arcs;
            return Self { vec: arc_range, fst, pos: arc_range.begin };
        }
    }


    impl<'a, ArcType: traits::Arc> traits::StateIterator<'a, ArcType, StateId, ConstFst<ArcType>>
    for StateIterator<'a, ArcType> {
        fn Value(&self) -> StateId {
            return self.pos as StateId;
        }

        fn Done(&self) -> bool {
            return self.pos == self.vec.len();
        }

        fn Next(&mut self) {
            self.pos += 1;
        }

        fn new(fst: &'a ConstFst<ArcType>) -> Self {
            return Self { vec: &fst.states, pos: 0 };
        }
    }
}

impl<'a, ArcType : 'a + traits::Arc> traits::IterableFst<'a, ArcType> for ConstFst<ArcType> {
    type ArcIterator = details::ArcIterator<'a, ArcType>;
    type StateIterator = details::StateIterator<'a, ArcType>;
}

impl<ArcType : traits::Arc> ConstFst<ArcType> {
    pub  fn new<'a, FST : traits::IterableFst<'a, ArcType>> (fst : &'a FST) -> Self {
        let mut result = Self {
            start : traits::kNoStateId,
            arcs : Vec::new(),
            states : Vec::new(),
        };


        let states = & mut result.states;
        use traits::StateIterator;
        use traits::ArcIterator;

        {
            let mut siter = generic::StateIterator::new(fst);
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
            let mut siter = generic::StateIterator::new(fst);
            while !siter.Done() {
                let state = siter.Value() as usize;
                let arcs_begin = result.arcs.len();
                let mut aiter = generic::ArcIterator::new(fst,siter.Value());
                while !aiter.Done() {
                    result.arcs.push(aiter.Value());
                    aiter.Next();
                }
                let arcs_end = result.arcs.len();
                let arcs = ArcRange{begin : arcs_begin, end : arcs_end};
                let weight = fst.Final(siter.Value() );
                states[state] = StateDescription{arcs, final_weight : weight};
                siter.Next();
            }
        }
        result.start = fst.Start();
        result.arcs.shrink_to_fit();
        return result;
    }

    pub fn Final(&self, state : StateId) -> ArcType::Weight {
        return self.states[state as usize].final_weight;
    }
    pub fn Start(&self) -> StateId {
        return self.start;
    }

    pub fn NumStates(&self) -> StateId {
        return self.states.len() as StateId;
    }
    pub fn NumArcs(&self, state : StateId) -> isize {
        let state_descr = &self.states[state as usize];
        let arc_indexer = &state_descr.arcs;
        return (arc_indexer.end - arc_indexer.begin) as isize;
    }
}