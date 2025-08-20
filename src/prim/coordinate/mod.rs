use super::*;

pub mod point2d;
pub mod point3d;
pub mod point4d;

pub trait Coordinate<const A: u8, B, C> 
where
    B: num::Int,
    B: num::Prim,
    C: q::Engine {
    fn distance_between(self, rhs: Self) -> q::Q<A, B, C>;
}