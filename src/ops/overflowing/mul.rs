pub trait OverflowingMul
where
    Self: Sized {
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
}