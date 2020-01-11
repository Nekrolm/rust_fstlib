use super::weight::Weight;

pub type Label = i32;
pub type StateId = i32;

pub trait Arc : Clone + Copy {
    type Weight : Weight;
    type Label;
    type StateId;

    fn new(ilabel : Self::Label,
           olabel : Self::Label,
           weight : Self::Weight,
           nextstate : Self::StateId) -> Self;

    fn ilabel(&self) -> Self::Label;
    fn olabel(&self) -> Self::Label;
    fn weight(&self) -> Self::Weight;
    fn nextstate(&self) -> Self::StateId;
}






