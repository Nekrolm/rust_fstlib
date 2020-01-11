use super::super::traits;

pub fn Plus<W : traits::Weight>(lhs : W, rhs : W) -> W {
    return W::Plus(lhs, rhs);
}

pub fn Times<W : traits::Weight>(lhs : W, rhs : W) -> W {
    return W::Times(lhs, rhs);
}