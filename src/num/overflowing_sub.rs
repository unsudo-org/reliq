pub trait OverflowingSub
where
    Self: Sized {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
}