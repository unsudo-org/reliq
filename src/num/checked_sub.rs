pub trait CheckedSub
where
    Self: Sized {
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}