use super::*;

#[derive(Clone)]
#[derive(Copy)]
pub struct Point2D<const A: u8, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub x: q::Q<A, B, q::DefaultMode, C>,
    pub y: q::Q<A, B, q::DefaultMode, C>
}

impl<const A: u8, B, C> From<(B, B)> for Point2D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: (B, B)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into()
        }
    }
}

impl<const A: u8, B, C> From<(q::Q<A, B, q::DefaultMode, C>, q::Q<A, B, q::DefaultMode, C>)> for Point2D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: (q::Q<A, B, q::DefaultMode, C>, q::Q<A, B, q::DefaultMode, C>)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}