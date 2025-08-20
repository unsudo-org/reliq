pub trait CheckedMul
where
    Self: Sized {
    fn checked_mul(self, rhs: Self) -> Option<Self>;
}