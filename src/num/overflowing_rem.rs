pub trait OverflowingRem
where
    Self: Sized {
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);
}