use crate::ops::ToPrim;

use super::*;


// MARK: HexMode

#[derive(Clone)]
#[derive(Copy)]
pub struct HexMode {
    code: u32
}

impl Mode for HexMode {}


// MARK: Hex

pub type Hex<const A: u8, B > = Color<A, B, HexMode>;

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

impl<const A: u8, B> From<Rgb<A, B>> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
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

impl<const A: u8, B> TryFrom<Hsl<A, B>> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    type Error = Error;

    fn try_from(value: Hsl<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let n_0_0: q::Q<A, B> = n(0_0)?;
        let n_1_0: q::Q<A, B> = n(1_0)?;
        let n_2_0: q::Q<A, B> = n(2_0)?;
        let n_60_0: q::Q<A, B> = n(60_0)?;
        let n_255_0: q::Q<A, B> = n(255_0)?;
        let hsl: Hsl<A, B> = value;
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
        let r: q::Q<A, B> = (r + m)?;
        let r: q::Q<A, B> = (r * n_255_0)?;
        let r: q::Q<A, B> = r.round_down().clamp(n_0_0, n_255_0);
        let r: u32 = r.to_u32()?;
        let r: u32 = r << 16;
        let g: q::Q<A, B> = (g + m)?;
        let g: q::Q<A, B> = (g * n_255_0)?;
        let g: q::Q<A, B> = g.round_down().clamp(n_0_0, n_255_0);
        let g: u32 = g.to_u32()?;
        let g: u32 = g << 8;
        let b: q::Q<A, B> = (b + m)?;
        let b: q::Q<A, B> = (b * n_255_0)?;
        let b: q::Q<A, B> = b.round_down().clamp(n_0_0, n_255_0);
        let b: u32 = b.to_u32()?;
        let code: u32 = r | g;
        let code: u32 = code | b;
        let ret: Self = code.into();
        Ok(ret)
    }
}

impl<const A: u8, B> TryFrom<Hsla<A, B>> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    type Error = Error;

    fn try_from(value: Hsla<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };    
        let n_1_0: q::Q<A, B> = n(1_0)?;
        let hsla: Hsla<_, _> = value;
        let a: q::Q<A, B> = hsla.a();
        if a != n_1_0 {
            return Err(Error::AlphaTruncation)
        }
        let hsl: Hsl<A, B> = hsla.into();
        let ret: Self = hsl.try_into()?;
        Ok(ret)
    }
}

impl<const A: u8, B> TryFrom<Rgba<A, B>> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    type Error = Error;

    fn try_from(value: Rgba<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };      
        let rgba: Rgba<A, B> = value;
        let r: u8 = rgba.r();
        let g: u8 = rgba.g();
        let b: u8 = rgba.b();
        let a: q::Q<A, B> = rgba.a();
        if a != n(0_0)? {
            return Err(Error::AlphaTruncation)
        }
        let code: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        let ret: Hex<A, B> = Color {
            mode: HexMode {
                code
            },
            m_0: ::core::marker::PhantomData
        };
        Ok(ret)
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
    (): q::Supported<A, B>,
    (): q::Supported<1, B>
    {}