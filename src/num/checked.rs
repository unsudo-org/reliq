use super::*;

pub trait Checked 
where
    Self: CheckedAdd,
    Self: CheckedSub,
    Self: CheckedMul,
    Self: CheckedDiv,
    Self: CheckedRem,
    Self: CheckedShl,
    Self: CheckedShr,
    Self: CheckedNeg {}

impl<T> Checked for T
where
    T: CheckedAdd,
    T: CheckedSub,
    T: CheckedMul,
    T: CheckedDiv,
    T: CheckedRem,
    T: CheckedShl,
    T: CheckedShr,
    T: CheckedNeg {}