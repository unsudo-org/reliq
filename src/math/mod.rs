use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum SigmoidCurve {
    Logistic,
    TanH,
    ArcTan,
    Soft,
    Hard,
    Swish,
    Gelu
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Sigmoid<const A: u8, B> 
where
    B: ops::Int,
    B: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    curve: SigmoidCurve,
    steepness: q::Q<A, B>,
    shift: q::Q<A, B>,
    scale: q::Q<A, B>,
    offset: q::Q<A, B>
}

impl<const A: u8, B> Default for Sigmoid<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    fn default() -> Self {
        Self {
            curve: SigmoidCurve::Logistic,
            steepness: q::as_1(),
            shift: q::as_0(),
            scale: q::as_1(),
            offset: 
        }
    }
}




pub mod linear_reg;