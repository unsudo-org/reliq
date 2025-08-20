use super::*;

pub trait Float
where
    Self: Sized,
    Self: Num,
    Self: QuickAccess {}

impl Float for f32 {}
impl Float for f64 {}