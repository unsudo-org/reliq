use super::*;

pub trait Int 
where
    Self: Sized,
    Self: Num,
    Self: Checked,
    Self: Overflowing,
    Self: Eq,
    Self: PartialEq,
    Self: Bits,
    Self: Bound,
    Self: QuickAccess {}