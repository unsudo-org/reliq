use super::*;

pub trait Int 
where
    Self: ops::Int,
    Self: ops::Prim {}

pub trait Timestamp 
where
    Self: ops::Int,
    Self: ops::Prim,
    Self: ops::Unsigned {}

impl<T> Int for T
where
    T: ops::Int,
    T: ops::Prim {}

impl<T> Timestamp for T
where
    Self: ops::Int,
    Self: ops::Prim,
    Self: ops::Unsigned {}