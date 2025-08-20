pub trait CheckedShr
where
    Self: Sized {
    fn checked_shr(self, rhs: Self) -> Option<Self>;
}