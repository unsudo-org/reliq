pub trait OverflowingAdd
where
    Self: Sized {
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
}