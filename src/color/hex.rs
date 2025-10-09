use super::*;

#[derive(Clone)]
#[derive(Copy)]
pub struct HexMode {
    code: u32
}

impl Mode for HexMode {}

pub type Hex<const A: u8 = 1, B = usize> = Color<A, B, HexMode>;

impl<const A: u8, B> Default for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn default() -> Self {
        0x000000u32.into()
    }
}

impl<const A: u8, B> From<Rgba<A, B>> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: Rgba<A, B>) -> Self {
        let rgba: Rgba<A, B> = value;
        let r: u8 = rgba.r();
        let g: u8 = rgba.g();
        let b: u8 = rgba.b();
        let code: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        Self {
            mode: HexMode {
                code
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> From<Rgb<A, B>> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn from(value: Rgb<A, B>) -> Self {
        let rgb: Rgb<A, B> = value;
        let r: u8 = rgb.r();
        let g: u8 = rgb.g();
        let b: u8 = rgb.b();
        let code: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        Self {
            mode: HexMode {
                code
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> From<(u8, u8, u8)> for Hex<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn from(value: (u8, u8, u8)) -> Self {
        let rgb: Rgb<A, B> = value.into();
        let ret: Self = rgb.into();
        ret
    }
}

impl<const A: u8, B> From<u8> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: u8) -> Self {
        let code: u8 = value;
        let code: u32 = code.into();
        Self {
            mode: HexMode {
                code
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> From<u16> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: u16) -> Self {
        let code: u16 = value;
        let code: u32 = code.into();
        Self {
            mode: HexMode {
                code
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> From<u32> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: u32) -> Self {
        let code: u32 = value;
        let code: u32 = code.clamp(0x000000, 0xFFFFFF);
        Self {
            mode: HexMode {
                code
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> TryFrom<Hsla<A, B>> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Error = Error;

    fn try_from(value: Hsla<A, B>) -> ::core::result::Result<Self, Self::Error> {
        
    }
}

impl<const A: u8, B> TryFrom<Hsl<A, B>> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Error = Error;

    fn try_from(value: Hsl<A, B>) -> ::core::result::Result<Self, Self::Error> {
        
    }
}

impl<const A: u8, B> ::core::ops::Deref for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Target = u32;
    
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.mode.code
    }
}

impl<const A: u8, B> CommonExt<A, B> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> 
    {}