use super::*;

pub type Hsl<const A: u8 = 1, B = usize, C = q::DefaultEngine, D = DefaultEngine> = Color<A, B, HslMode<A, B, C>, C, D>;

impl<const A: u8, B, C, D> Hsl<A, B, C, D> 
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub fn h(&self) -> &u16 {
        &self.mode.h
    }

    pub fn s(&self) -> &q::Unit<A, B, C> {
        &self.mode.s
    }

    pub fn l(&self) -> &q::Unit<A, B, C> {
        &self.mode.l
    }
}

pub struct HslMode<const A: q::Precision, B, C = q::DefaultEngine>
where
    B: ops::Int,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    h: u16,
    s: q::Unit<A, B, C>,
    l: q::Unit<A, B, C>
}

impl<const A: q::Precision, B, C> Mode for HslMode<A, B, C>
where
    B: ops::Int,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {}