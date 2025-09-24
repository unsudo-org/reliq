use super::*;

pub type Hex<const A: u8 = 1, B = usize, C = q::DefaultEngine, D = DefaultEngine> = Color<A, B, HexMode, C, D>;

impl<const A: u8, B, C, D> From<Rgb<A, B, C, D>> for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: Rgb<A, B, C, D>) -> Self {
        let rgb: Rgb<A, B, C, D> = value;
        let r: u8 = *rgb.r();
        let g: u8 = *rgb.g();
        let b: u8 = *rgb.b();
        let code: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
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

impl<const A: q::Precision, B, C, D> From<Rgba<A, B, C, D>> for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: Rgba<A, B, C, D>) -> Self {
        let rgba: Rgba<A, B, C, D> = value;
        let r: u8 = *rgba.r();
        let g: u8 = *rgba.g();
        let b: u8 = *rgba.b();
        let code: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
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

impl<const A: q::Precision, B, C, D> From<u8> for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
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
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        }
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
    #[inline]
    fn from(value: u16) -> Self {
        let code: u16 = value;
        let code: u32 = code.into();
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

impl<const A: q::Precision, B, C, D> From<u32> for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
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
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C, D> TryFrom<Hsl<A, B, C, D>> for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Error = Error;

    #[inline]
    fn try_from(value: Hsl<A, B, C, D>) -> Result<Self> {
        use ops::ToPrim;
        let as_0: q::Unit<A, B, C> = q::as_0::<A, B, _, C>();
        let as_1: q::Unit<A, B, C> = q::as_1::<A, B, _, C>();
        let as_2: q::Unit<A, B, C> = q::as_2::<A, B, _, C>();
        #[allow(clippy::inconsistent_digit_grouping)]
        let as_255: q::Unit<A, B, C> = q::r#as::<1, _, u16, _, _, _, _>(255_0_u16)?;
        let hsl: Hsl<A, B, C, D> = value;
        let h: u16 = *hsl.h();
        let h: u128 = h.into();
        let h: u128 = h.clamp(B::MIN_AS_U128, B::MAX_AS_U128);
        let h: u128 = h * 10;
        let h: q::Unit<A, B, C> = q::r#as::<1, _, u128, _, _, _, _>(h)?;
        let s: q::Unit<A, B, C> = *hsl.s();
        let l: q::Unit<A, B, C> = *hsl.l();
        let c: q::Unit<A, B, C> = (as_2 * l)?;
        let c: q::Unit<A, B, C> = (c - as_1)?;
        let c: q::Unit<A, B, C> = c.abs();
        let c: q::Unit<A, B, C> = (as_1 - c)?;
        let c: q::Unit<A, B, C> = (c * s)?;
        let x: q::Unit<A, B, C> = (h % as_2)?;
        let x: q::Unit<A, B, C> = (x - as_1)?;
        let x: q::Unit<A, B, C> = x.abs();
        let x: q::Unit<A, B, C> = (as_1 - x)?;
        let x: q::Unit<A, B, C> = (c * x)?;
        let hsec: u32 = h.round_down().to_u32()?;
        let (r, g, b) = match hsec {
            0 => (c, x, as_0),
            1 => (x, c, as_0),
            2 => (as_0, c, x),
            3 => (as_0, x, c),
            4 => (x, as_0, c),
            _ => (c, as_0, x)
        };
        let m: q::Unit<A, B, C> = (c / as_2)?;
        let m: q::Unit<A, B, C> = (l - m)?;
        let r: q::Unit<A, B, C> = (r + m)?;
        let r: q::Unit<A, B, C> = (r * as_255)?;
        let g: q::Unit<A, B, C> = (g + m)?;
        let g: q::Unit<A, B, C> = (g * as_255)?;
        let b: q::Unit<A, B, C> = (b + m)?;
        let b: q::Unit<A, B, C> = (b * as_255)?;
        let r: u8 = r.to_u8()?;
        let g: u8 = g.to_u8()?;
        let b: u8 = b.to_u8()?;
        let code: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        Ok(Self {
            mode: HexMode {
                code
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        })
    }
}

impl<const A: u8, B, C, D> TryFrom<Hsla<A, B, C, D>> for Hex<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Error = Error;

    fn try_from(value: Hsla<A, B, C, D>) -> Result<Self> {
        
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
    
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.mode.code
    }
}

pub struct HexMode {
    code: u32
}

impl Mode for HexMode {}