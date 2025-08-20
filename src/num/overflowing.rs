use super::*;

pub trait Overflowing
where
    Self: OverflowingAdd,
    Self: OverflowingSub,
    Self: OverflowingMul,
    Self: OverflowingDiv,
    Self: OverflowingRem,
    Self: OverflowingShl,
    Self: OverflowingShr,
    Self: OverflowingNeg {}