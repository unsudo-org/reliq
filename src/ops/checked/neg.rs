pub trait CheckedNeg
where
    Self: Sized {
    fn checked_neg(self, rhs: Self) -> Option<Self>;
}