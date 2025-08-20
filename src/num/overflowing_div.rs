pub trait OverflowingDiv
where
    Self: Sized {
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
}