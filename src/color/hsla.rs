use super::*;

pub type Hsla<const A: u8 = 1, B = usize, C = q::DefaultEngine, D = DefaultEngine> = Color<A, B, HslaMode<A, B, C>, C, D>;

impl<const A: u8, B, C, D> Hsla<A, B, C, D> 
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>  {
    #[inline]
    pub fn h(&self) -> &u16 {
        &self.mode.h
    }

    #[inline]
    pub fn s(&self) -> &q::Unit<A, B, C> {
        &self.mode.s
    }

    #[inline]
    pub fn l(&self) -> &q::Unit<A, B, C> {
        &self.mode.l
    }

    #[inline]
    pub fn a(&self) -> &q::Unit<A, B, C> {
        &self.mode.a
    }
}

pub struct HslaMode<const A: u8 = 1, B = usize, C = q::DefaultEngine>
where
    B: ops::Int,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    h: u16,
    s: q::Unit<A, B, C>,
    l: q::Unit<A, B, C>,
    a: q::Unit<A, B, C>
}

impl<const A: u8, B, C> Mode for HslaMode<A, B, C>
where
    B: ops::Int,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {}