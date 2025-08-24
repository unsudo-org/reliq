use super::*;

type Q<const A: Precision, B, C> = q::Q<A, B, q::DefaultMode, C>;
type Precision = u8;

pub trait SignedPoint<const A: Precision, B, C, D> 
where
    Self: Sized,
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn distance_between(self, rhs: Self) -> Result<Q<A, B, C>, D>;
}