use super::super::traits;
use traits::Weight;
use traits::Label;
use traits::StateId;



#[derive(Copy, Clone)]
pub struct Arc<W : Weight> {
    pub ilabel : Label,
    pub olabel : Label,
    pub weight : W,
    pub nextstate : StateId
}

impl<W : Weight> traits::Arc for Arc<W> {
    type Weight = W;
    type Label = Label;
    type StateId = StateId;

    fn new(ilabel : Label,
           olabel: Label,
           weight : Self::Weight,
           nextstate : StateId) -> Self {
        return Arc{ilabel, olabel, weight, nextstate};
    }


    fn ilabel(&self) -> Self::Label {
        return self.ilabel;
    }
    fn olabel(&self) -> Self::Label{
        return self.olabel;
    }
    fn weight(&self) -> Self::Weight{
        return self.weight;
    }
    fn nextstate(&self) -> Self::StateId{
        return self.nextstate;
    }
}