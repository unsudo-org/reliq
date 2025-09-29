use super::*;

pub struct HslMode<const A: u8, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    h: u16,
    s: q::Unit<A, B>,
    l: q::Unit<A, B>
}

impl<const A: u8, B> Mode for HslMode<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>
    {}


pub type Hsl<const A: u8 = 1, B = usize> = Color<A, B, HslMode<A, B>>;

impl<const A: u8, B> Hsl<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub fn h(&self) -> u16 {
        self.mode.h
    }

    pub fn s(&self) -> q::Unit<A, B> {
        self.mode.s
    }

    pub fn l(&self) -> q::Unit<A, B> {
        self.mode.l
    }
}