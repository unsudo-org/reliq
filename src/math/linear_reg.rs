use super::*;

pub type Precision = u8;
pub type Len = usize;
pub type Dimension = usize;

pub struct PredictionEngine<
    const A: Precision,
    const B: Len,
    const C: Dimension,
          D
>
where
    D: ops::Int,
    D: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<D>,
    (): q::Supported<A, D> {
    dataset: array::Array<B, point::Point<A, C, D>>
}

impl<
    const A: Precision,
    const B: Len,
    const C: Dimension,
          D
> PredictionEngine<A, B, C, D>
where
    D: ops::Int,
    D: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<D>,
    (): q::Supported<A, D> {
    pub fn preview(&self) {
        
    }
}