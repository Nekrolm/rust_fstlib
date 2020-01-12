
pub trait Semiring {
    fn Plus(lhs : Self, rhs : Self) -> Self;
    fn Times(lhs : Self, rhs : Self) -> Self;

    fn Zero() -> Self;
    fn One() -> Self;
}


pub trait Weight : Clone + Copy {
    type ValueType : Semiring;

    fn new(value : Self::ValueType) -> Self;

    fn Value(&self) -> Self::ValueType;
    fn SetValue(& mut self, v : Self::ValueType);

    fn Plus(lhs : Self, rhs : Self) -> Self {
        return Weight::new(<Self::ValueType as Semiring>::Plus(lhs.Value(), rhs.Value()));
    }

    fn Times(lhs : Self, rhs : Self) -> Self {
        return Weight::new(<Self::ValueType as Semiring>::Times(lhs.Value(), rhs.Value()));
    }


    fn Zero() -> Self {
        return Weight::new( <Self::ValueType as Semiring>::Zero()  );
    }
    fn One() -> Self {
        return Weight::new( <Self::ValueType as Semiring>::One()  );
    }
}
