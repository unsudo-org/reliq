pub trait CheckedAdd 
where
    Self: Sized {
    fn checked_add(self, rhs: Self) -> Option<Self>;
}