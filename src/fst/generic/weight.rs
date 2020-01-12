use super::super::traits;

#[derive(Copy, Clone)]
pub struct Weight<ValueT : Copy + Clone + traits::Semiring> {
    value : ValueT
}

impl<ValueT : Copy + Clone + traits::Semiring> Weight<ValueT> {
    pub fn new(value : ValueT) -> Self {
        Weight{value}
    }

    pub fn Value(&self) -> ValueT {
        return self.value;
    }
    pub fn SetValue(&mut self, v : ValueT) {
        self.value = v;
    }

    pub fn Plus(lhs : Self, rhs : Self) -> Self {
        return Weight::new(ValueT::Plus(lhs.Value(), rhs.Value()));
    }

    pub fn Times(lhs : Self, rhs : Self) -> Self {
        return Weight::new(ValueT::Times(lhs.Value(), rhs.Value()));
    }


    pub fn Zero() -> Self {
        return Weight::new( ValueT::Zero()  );
    }
    pub fn One() -> Self {
        return Weight::new( ValueT::One()  );
    }
}


impl<ValueT> traits::Weight for Weight<ValueT>
where ValueT : Copy + Clone + traits::Semiring {
    type ValueType = ValueT;

    fn new(value : Self::ValueType) -> Self {
        Weight{value}
    }

    fn Value(&self) -> Self::ValueType {
        return self.Value();
    }
    fn SetValue(&mut self, v : Self::ValueType) {
        self.SetValue(v);
    }
}



pub fn Plus<W : traits::Weight>(lhs : W, rhs : W) -> W {
    return W::Plus(lhs, rhs);
}

pub fn Times<W : traits::Weight>(lhs : W, rhs : W) -> W {
    return W::Times(lhs, rhs);
}