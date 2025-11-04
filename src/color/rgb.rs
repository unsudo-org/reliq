use crate::ops::ToPrim;

use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct RgbMode {
    r: u8,
    g: u8,
    b: u8
}

impl Mode for RgbMode {}

pub type Rgb<const A: u8, B> = Color<A, B, RgbMode>;

impl<const A: u8, B> Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    pub fn r(&self) -> u8 {
        self.mode.r
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.mode.g
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.mode.b
    }
}

impl<const A: u8, B> Default for Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn default() -> Self {
        (0, 0, 0).into()
    }
}

impl<const A: u8, B> From<Hex<A, B>> for Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: Hex<A, B>) -> Self {
        let hex: Hex<A, B> = value;
        let r: u8 = ((*hex >> 16) & 0xFF) as u8;
        let g: u8 = ((*hex >> 8) & 0xFF) as u8;
        let b: u8 = (*hex & 0xFF) as u8;
        Self {
            mode: RgbMode {
                r,
                g,
                b
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> From<(u8, u8, u8)> for Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: (u8, u8, u8)) -> Self {
        let r: u8 = value.0;
        let g: u8 = value.1;
        let b: u8 = value.2;
        Self {
            mode: RgbMode {
                r,
                g,
                b
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> TryFrom<Hsl<A, B>> for Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    type Error = Error;

    fn try_from(value: Hsl<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let hsl: Hsl<A, B> = value;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let n_0_0: q::Q<A, B> = n(0_0)?;
        let n_1_0: q::Q<A, B> = n(1_0)?;
        let n_2_0: q::Q<A, B> = n(2_0)?;
        let n_60_0: q::Q<A, B> = n(60_0)?;
        let n_255_0: q::Q<A, B> = n(255_0)?;
        let h: u16 = hsl.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h_prime: q::Q<A, B> = (h / n_60_0)?;
        let h_mod: q::Q<A, B> = (h_prime % n_2_0)?;
        let h_mod: q::Q<A, B> = (h_mod - n_1_0)?;
        let s: q::Q<A, B> = hsl.s();
        let l: q::Q<A, B> = hsl.l();
        let c: q::Q<A, B> = (n_2_0 * l)?;
        let c: q::Q<A, B> = (c - n_1_0)?;
        let c: q::Q<A, B> = (n_1_0 - c)?;
        let c: q::Q<A, B> = (c * s)?;
        let x: q::Q<A, B> = (n_1_0 - h_mod)?;
        let x: q::Q<A, B> = (x * c)?;
        let (r, g, b) = {
            let segment: u32 = h_prime.to_u32()?;
            if segment == 0 {
                (c, x, n_0_0)
            } else if segment == 1 {
                (x, c, n_0_0)
            } else if segment == 2 {
                (n_0_0, c, x)
            } else if segment == 3 {
                (n_0_0, x, c)
            } else if segment == 4 {
                (x, n_0_0, c)
            } else {
                (c, n_0_0, x)
            }
        };
        let c_half: q::Q<A, B> = (c / n_2_0)?;
        let m: q::Q<A, B> = (l - c_half)?;
        
    }
}

impl<const A: u8, B> TryFrom<Hsla<A, B>> for Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    type Error = Error;
    
    fn try_from(value: Hsla<A, B>) -> ::core::result::Result<Self, Self::Error> {
        
    }
}

impl<const A: u8, B> TryFrom<Rgba<A, B>> for Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    type Error = Error;

    fn try_from(value: Rgba<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let r: u8 = value.r();
        let g: u8 = value.g();
        let b: u8 = value.b();
        let a: q::Q<A, B> = value.a();
        let as_1: q::Q<A, B> = q::as_1();
        if a != as_1 {
            return Err(Error::AlphaTruncation)
        }
        let ret: Self = Self {
            mode: RgbMode {
                r,
                g,
                b
            },
            m_0: ::core::marker::PhantomData
        };
        Ok(ret)   
    }
}

impl<const A: u8, B> CommonExt<A, B> for Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B>
    {}