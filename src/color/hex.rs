use super::*;

pub struct HexMode {
    code: u32
}

impl Mode for HexMode {}


pub type Hex<const A: u8 = 1, B = usize> = Color<A, B, HexMode>;

impl<const A: u8, B> Hex<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    pub fn lighten<C>(self, percentage: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>> {
        let percentage: q::Percentage<A, B> = percentage.into();
        self.interpolate::<(u8, u8, u8), _>((255, 255, 255), percentage)
    }

    #[inline]
    pub fn darken<C>(self, percentage: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>> {
        let percentage: q::Percentage<A, B> = percentage.into();
        self.interpolate::<(u8, u8, u8), _>((0, 0, 0), percentage)
    }

    #[inline]
    pub fn interpolate<C, D>(self, rhs: C, percentage: D) -> Result<Self>
    where
        C: Into<Self>,
        D: Into<q::Percentage<A, B>> {
        let rhs: Self = rhs.into();
        let percentage: q::Percentage<A, B> = percentage.into();
        let rgb: Rgb<A, B> = self.into();
        let rgb: Rgb<A, B> = rgb.interpolate(rhs, percentage)?;
        let ret: Self = rgb.into();
        Ok(ret)
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

impl<const A: u8, B> From<Rgba<A, B>> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: Rgba<A, B>) -> Self {
        let rgba: Rgba<A, B> = value;
        let r: u8 = *rgba.r();
        let g: u8 = *rgba.g();
        let b: u8 = *rgba.b();
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