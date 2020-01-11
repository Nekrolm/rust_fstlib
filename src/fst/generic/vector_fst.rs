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

    pub fn SetStart(&mut self, state : traits::StateId) {
        self.start = state;
    }
}

impl<ArcType : traits::Arc> traits::BaseFst<ArcType> for VectorFst<ArcType> {
    fn Final(&self, state : traits::StateId) -> ArcType::Weight {
        return self.states[state as usize].final_weight;
    }

    fn Start(&self) -> traits::StateId {
        return self.start;
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


pub struct VFstArcIterator<'a, ArcType : traits::Arc> {
    vec : &'a Vec<ArcType>,
    pos : usize
}

pub struct VFstStateIterator<'a, ArcType : traits::Arc> {
    vec : &'a Vec<StateDescription<ArcType>>,
    pos : usize
}

impl<'a, ArcType : 'a + traits::Arc> traits::IterableFst<'a, ArcType> for VectorFst<ArcType> {
    type ArcIterator = VFstArcIterator<'a, ArcType>;
    type StateIterator = VFstStateIterator<'a, ArcType>;
}


impl<'a, ArcType : traits::Arc> traits::ArcIterator<'a, ArcType, traits::StateId, VectorFst<ArcType>>
    for VFstArcIterator<'a, ArcType> {
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
for VFstStateIterator<'a, ArcType> {
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
//
//
