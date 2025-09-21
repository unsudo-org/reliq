use super::*;

pub type Hex<const A: q::Precision, B = usize, C = q::DefaultEngine, D = DefaultEngine> = Color<A, B, HexMode, C, D>;

pub struct HexMode {
    code: u32
}

impl Mode for HexMode {}

impl<const A: q::Precision, B, C, D> From<u8> for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: u8) -> Self {
        let code: u8 = value;
        let code: u32 = code.into();
        let ret: Self = code.into();
        ret
    }
}

impl<const A: q::Precision, B, C, D> From<u16> for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: u16) -> Self {
        let code: u16 = value;
        let code: u32 = code.into();
        let ret: Self = code.into();
        ret
    }
}

impl<const A: q::Precision, B, C, D> From<u32> for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: u32) -> Self {
        let code: u32 = value;
        let code: u32 = code.clamp(0x000000, 0xFFFFFF);
        Self {
            mode: HexMode {
                code
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        }
    }
}

impl<const A: q::Precision, B, C, D> ::core::ops::Deref for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.mode.code
    }
}