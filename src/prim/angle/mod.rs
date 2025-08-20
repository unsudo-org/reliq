use super::*;

pub mod deg;
pub mod rad;

pub trait Angle 
where
    Self: Sized {}