pub trait CheckedDiv
where
    Self: Sized {
    fn checked_div(self, rhs: Self) -> Option<Self>;
}