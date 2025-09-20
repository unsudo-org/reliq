use super::*;

pub type Hex<
    const A: q::Precision,
    B = usize,
    C = q::DefaultEngine
> = Color<
    A, 
    B, 
    HexMode, 
    C
>;

pub struct HexMode {
    code: u32
}

impl Mode for HexMode {}

impl<
    const A: q::Precision,
    B,
    C
> From<u16> for Hex<A, B, C>
where
    B: ops::Int,
    C: q::Engine,
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

impl<
    const A: q::Precision,
    B,
    C
> From<u32> for Hex<A, B, C>
where
    B: ops::Int,
    C: q::Engine,
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
            m_1: ::core::marker::PhantomData
        }
    }
}