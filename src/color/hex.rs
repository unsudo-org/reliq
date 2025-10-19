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

pub type LossyHex<const A: u8, B> = lossy::Lossy<Hex<A, B>>;

impl<const A: u8, B> From<lossy::Lossy<Rgba<A, B>>> for LossyHex<A, B>
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
        let ret: Hex<A, B> = Color {
            mode: HexMode {
                code
            },
            m_0: ::core::marker::PhantomData
        };
        let ret: lossy::Lossy<Color<A, B>> = lossy::Lossy::Trunc(ret);
        ret
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
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };      
        let h: u16 = value.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let s: q::Q<A, B> = value.s();
        let l: q::Q<A, B> = value.l();
        let a: q::Q<A, B> = value.a();
        if a != 0.into() {
            return Err(Error::AlphaTruncation)
        }
        let (r, g, b) = {
            let h: q::Q<A, B> = (h % n(360_0)?)?;
            let h: q::Q<A, B> = (h / n(360_0)?)?;
            let q: q::Q<A, B> = if l < n(0_5)? {
                (l * (n(1_0)? + s)?)?
            } else {
                let ret: q::Q<A, B> = (l * s)?;
                let ret: q::Q<A, B> = ((l + s)? - ret)?;
                ret
            };
            let p: q::Q<A, B> = n(2_0)?;
            let p: q::Q<A, B> = (p * l)?;
            let p: q::Q<A, B> = (p - q)?;
        
            let hue_to_rgb = |
                p: q::Q<A, B>,
                q: q::Q<A, B>,
                mut t: q::Q<A, B>
            | -> Result<q::Q<A, B>> {
                if t < n(0_0)? {
                    t = (t + n(1_0)?)?;
                }
                if t > n(1_0)? {
                    t = (t - n(1_0)?)?;
                }
                if t < (n(1_0)? / n(6_0)?)? {
                    return p + (q - p) * 6.0 * t
                }
                if t < (n(1_0)? / n(2_0)?)? {
                    return q
                }
                if t < (n(2_0)? / n(3_0)?)? {
                    return p + (q - p) * (2.0 / 3.0 - t) * 6.0 
                }
                Ok(p)
            };
            let r: q::Q<A, B> = hue_to_rgb(p, q, (h + (n(1_0)? / n(3_0)?)?)?)?;
            let r: u32 = r.try_into().ok().ok_or(Error::UnsupportedConversion)?;
            let g: q::Q<A, B> = hue_to_rgb(p, q, h)?;
            let b: q::Q<A, B> = hue_to_rgb(p, q, (h - (n(1_0)? / n(3_0)?)?)?)?;
            
        };
        Ok(())
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

impl<const A: u8, B> CommonAlphaExt<A, B> for Hex<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> 
    {}