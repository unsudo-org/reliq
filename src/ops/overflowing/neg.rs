pub trait OverflowingNeg
where
    Self: Sized {
    fn overflowing_neg(self, rhs: Self) -> (Self, bool);
}