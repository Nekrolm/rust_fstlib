
pub trait Weight : Clone + Copy {
    type ValueType;

    fn new(value : Self::ValueType) -> Self;

    fn Value(&self) -> Self::ValueType;
    fn SetValue(& mut self, v : Self::ValueType);

    fn Plus(lhs : Self, rhs : Self) -> Self;
    fn Times(lhs : Self, rhs : Self) -> Self;


    fn Zero() -> Self;
    fn One() -> Self;
}
