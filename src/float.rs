use super::*;

pub trait Float
where
    Self: Sized,
    Self: num::Num
{}

impl Float for f32 {}
impl Float for f64 {}