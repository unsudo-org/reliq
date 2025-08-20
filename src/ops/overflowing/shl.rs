pub trait OverflowingShl
where
    Self: Sized {
    fn overflowing_shl(self, rhs: Self) -> (Self, bool);
}