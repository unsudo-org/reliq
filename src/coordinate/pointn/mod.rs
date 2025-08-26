use super::*;

type Q<const A: q::Precision, B, C> = q::Q<A, B, q::DefaultMode, C>;
type Count = usize;

#[repr(transparent)]
pub struct PointN<const A: q::Precision, const B: Count, C, D = q::DefaultEngine>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    n: array::Array<B, Q<A, C, D>>
}

impl<const A: q::Precision, const B: Count, C, D> PointN<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub fn dimension(&self) -> Option<Q<A, C, D>> {

    }
}