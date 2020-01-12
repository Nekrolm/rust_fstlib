use super::super::traits;

struct StateDescription<ArcType : traits::Arc> {
    arcs : Vec<ArcType>,
    final_weight : ArcType::Weight,
}



pub struct VectorFst<ArcType: traits::Arc> {
    start : traits::StateId,
    states : Vec<StateDescription<ArcType>>
}



impl <ArcType : traits::Arc> VectorFst<ArcType> {
    pub fn new() -> Self {
        return Self{start : traits::kNoStateId, states : Vec::new()};
    }

    pub fn AddState(&mut self) -> traits::StateId {
        use traits::Weight;
        let new_state_id = self.states.len();
        let new_state = StateDescription{arcs : Vec::new(), final_weight : ArcType::Weight::Zero() };
        self.states.push(new_state);
        return new_state_id as traits::StateId;
    }

    pub fn AddArc(&mut self, state : traits::StateId, arc : ArcType) {
        self.states[state as usize].arcs.push(arc);
    }

    pub fn SetFinal(&mut self, state : traits::StateId, w : ArcType::Weight) {
        self.states[state as usize].final_weight = w;
    }

    pub fn DeleteStates(&mut self) {
        self.states.clear();
        self.start = traits::kNoStateId;
    }

    pub fn DeleteArcs(&mut self, s : traits::StateId) {
        self.states[s as usize].arcs.clear();
    }

    pub fn SetStart(&mut self, state : traits::StateId) {
        self.start = state;
    }

    pub fn Final(&self, state : traits::StateId) -> ArcType::Weight {
        return self.states[state as usize].final_weight;
    }

    pub fn Start(&self) -> traits::StateId {
        return self.start;
    }
}

impl<ArcType : traits::Arc> traits::BaseFst<ArcType> for VectorFst<ArcType> {
    fn Final(&self, state : traits::StateId) -> ArcType::Weight {
        return self.Final(state);
    }

    fn Start(&self) -> traits::StateId {
        return self.Start();
    }
}

impl<ArcType : traits::Arc> traits::ExpandedFst<ArcType> for VectorFst<ArcType> {
    fn NumStates(&self) -> traits::StateId {
        return self.states.len() as traits::StateId;
    }
    fn NumArcs(&self, state : traits::StateId) -> isize {
        return self.states[state as usize].arcs.len() as isize;
    }
}

impl<ArcType : traits::Arc> traits::MutableFst<ArcType> for VectorFst<ArcType> {
    fn SetStart(&mut self, s : traits::StateId) {
        self.SetStart(s);
    }
    fn SetFinal(&mut self, s : traits::StateId, w : ArcType::Weight) {
        self.SetFinal(s, w);
    }
    fn AddState(&mut self) -> traits::StateId {
        return self.AddState();
    }
    fn AddArc(&mut self, s : traits::StateId, arc : ArcType) {
        self.AddArc(s, arc);
    }
    fn DeleteStates(&mut self) {
        self.DeleteStates();
    }
    fn DeleteArcs(&mut self, s : traits::StateId){
        self.DeleteArcs(s);
    }
}

mod details {

    use super::traits as traits;
    use super::VectorFst;

    pub struct ArcIterator<'a, ArcType: traits::Arc> {
        vec: &'a Vec<ArcType>,
        pos: usize
    }

    pub struct StateIterator<'a, ArcType: traits::Arc> {
        vec: &'a Vec<super::StateDescription<ArcType>>,
        pos: usize
    }

    impl<'a, ArcType : traits::Arc> traits::ArcIterator<'a, ArcType, traits::StateId, VectorFst<ArcType>>
    for ArcIterator<'a, ArcType> {
        fn Value(&self) -> ArcType {
            return self.vec[self.pos]
        }

        fn Done(&self) -> bool {
            return self.pos == self.vec.len();
        }

        fn Next(&mut self) {
            self.pos += 1;
        }

        fn new (fst: &'a VectorFst<ArcType>, state : traits::StateId) -> Self {
            return Self{ vec : &fst.states[state as usize].arcs, pos : 0 };
        }
    }


    impl<'a, ArcType : traits::Arc> traits::StateIterator<'a, ArcType, traits::StateId, VectorFst<ArcType>>
    for StateIterator<'a, ArcType> {
        fn Value(&self) -> traits::StateId {
            return self.pos as traits::StateId;
        }

        fn Done(&self) -> bool {
            return self.pos == self.vec.len();
        }

        fn Next(&mut self) {
            self.pos += 1;
        }

        fn new (fst: &'a VectorFst<ArcType>) -> Self {
            return Self{ vec : &fst.states, pos : 0 };
        }
    }

}

impl<'a, ArcType : 'a + traits::Arc> traits::IterableFst<'a, ArcType> for VectorFst<ArcType> {
    type ArcIterator = details::ArcIterator<'a, ArcType>;
    type StateIterator = details::StateIterator<'a, ArcType>;
}



//
//
