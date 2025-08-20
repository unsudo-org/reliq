pub trait OverflowingShr
where
    Self: Sized {
    fn overflowing_shr(self, rhs: Self) -> (Self, bool);
}