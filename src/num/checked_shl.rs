pub trait CheckedShl
where
    Self: Sized {
    fn checked_shl(self, rhs: Self) -> Option<Self>;
}