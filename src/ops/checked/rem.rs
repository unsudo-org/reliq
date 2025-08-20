pub trait CheckedRem
where
    Self: Sized {
    fn checked_rem(self, rhs: Self) -> Option<Self>;
}