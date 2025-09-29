use super::*;

pub struct HslaMode<const A: u8 = 1, B = usize>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    h: u16,
    s: q::Q<A, B>,
    l: q::Q<A, B>,
    a: q::Q<A, B>
}

impl<const A: u8, B> Mode for HslaMode<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {}


pub type Hsla<const A: u8 = 1, B = usize> = Color<A, B, HslaMode<A, B>>;

impl<const A: u8, B> Hsla<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>  {
    #[inline]
    pub fn h(&self) -> &u16 {
        &self.mode.h
    }

    #[inline]
    pub fn s(&self) -> &q::Q<A, B> {
        &self.mode.s
    }

    #[inline]
    pub fn l(&self) -> &q::Q<A, B> {
        &self.mode.l
    }

    #[inline]
    pub fn a(&self) -> &q::Q<A, B> {
        &self.mode.a
    }
}