use super::*;

::modwire::expose!(
    pub error
);

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Color<const A: u8, B, C = q::DefaultEngine> 
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    Hex(u32),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, q::Q<A, B, q::DefaultMode, C>)
}

impl<const A: u8, B, C> Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    pub fn is_hex(self) -> bool {
        matches!(self, Self::Hex(_))
    }

    #[inline]
    pub fn is_rgb(self) -> bool {
        matches!(self, Self::Rgb(_, _, _))
    }

    #[inline]
    pub fn is_rgba(self) -> bool {
        matches!(self, Self::Rgba(_, _, _, _))
    }

    #[inline]
    pub fn interpolate<D, E>(self, rhs: D, t: E) -> Result<Self>
    where
        D: Into<Self>,
        E: Into<q::Q<A, B, q::DefaultMode, C>> {
        let rhs: Self = rhs.into();
        let t: q::Q<A, B, q::DefaultMode, C> = t.into();
        match (self, rhs) {
            (Self::Rgb(r_0, g_0, b_0), Self::Rgb(r_1, g_1, b_1)) => {
                let lerp: _ = |a: u8, b: u8| -> q::Result<u8> {
                    let a: B = a
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedOperation)?;
                    let a: q::Q<A, B, q::DefaultMode, C> = a.into();
                    let b: B = b
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedOperation)?;
                    let b: q::Q<A, B, q::DefaultMode, C> = b.into();
                    let ret: q::Q<A, B, q::DefaultMode, C> = ((a + (b - a)?)? * t)?;
                    let ret: B = ret.as_int();
                    let ret: u8 = ret
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedOperation)?;
                    Ok(ret)
                };
                let r: u8 = lerp(r_0, r_1)?;
                let g: u8 = lerp(g_0, g_1)?;
                let b: u8 = lerp(b_0, b_1)?;
                Ok(Self::Rgb(r, g, b))
            },
            (Self::Rgba(r_0, g_0, b_0, a_0), Self::Rgba(r_1, g_1, b_1, a_1)) => {
                let lerp: _ = |a: u8, b: u8| -> q::Result<u8> {
                    let a: B = a
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedOperation)?;
                    let a: q::Q::<A, B, q::DefaultMode, C> = a.into();
                    let b: B = b
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedOperation)?;
                    let b: q::Q::<A, B, q::DefaultMode, C> = b.into();
                    let ret: q::Q<A, B, q::DefaultMode, C> = ((a + (b - a)?)? * t)?;
                    let ret: B = ret.as_int();
                    let ret: u8 = ret
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedOperation)?;
                    Ok(ret)
                };
                let a: q::Q<A, B, q::DefaultMode, C> = {
                    let ret: q::Q<A, B, q::DefaultMode, C> = ((a_0 + (a_1 - a_0)?)? * t)?;
                    let ret: q::Q<A, B, q::DefaultMode, C> = ret.clamp(q::as_0(), q::as_1());
                    ret
                };
                let r: u8 = lerp(r_0, r_1)?;
                let g: u8 = lerp(g_0, g_1)?;
                let b: u8 = lerp(b_0, b_1)?;
                Ok(Self::Rgba(r, g, b, a))
            },
            (Self::Hex(hex_0), Self::Hex(hex_1)) => {
                let r_0: u8 = ((hex_0 >> 16) & 0xff) as u8;
                let g_0: u8 = ((hex_0 >> 8) & 0xff) as u8;
                let b_0: u8 = (hex_0 & 0xff) as u8;
                let r_1: u8 = ((hex_1 >> 16) & 0xff) as u8;
                let g_1: u8 = ((hex_1 >> 8) & 0xff) as u8;
                let b_1: u8 = (hex_1 & 0xff) as u8;
                let rgb: Self = (r_0, g_0, b_0).into();
                let ret: Self = rgb.interpolate((r_1, g_1, b_1), t)?;
                Ok(ret)
            },
            (lhs, rhs) => {
                let lhs: Self = match lhs {
                    Self::Rgb(r, g, b) => Self::Rgba(r, g, b, q::as_1()),
                    o => o
                };
                let rhs: Self = match rhs {
                    Self::Rgb(r, g, b) => Self::Rgba(r, g, b, q::as_1()),
                    o => o
                };
                let ret: Self = lhs.interpolate(rhs, t)?;
                Ok(ret)
            }
        }
    }

    #[inline]
    pub fn as_hex(self) -> Self {
        match self {
            Self::Rgb(r, g, b) => {
                let ret: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                let ret: Self = Self::Hex(ret);
                ret
            },
            Self::Rgba(r, g, b, _) => {
                let ret: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                let ret: Self = Self::Hex(ret);
                ret
            },
            Self::Hex(_) => self
        }
    }

    #[inline]
    pub fn as_rgb(self) -> Self {
        match self {
            Self::Hex(hex) => {
                let (r, g, b) = (
                    ((hex >> 16) & 0xff) as u8,
                    ((hex >> 8) & 0xff) as u8,
                    (hex & 0xff) as u8
                );
                Self::Rgb(r, g, b)
            },
            Self::Rgba(r, g, b, _) => Self::Rgb(r, g, b),
            Self::Rgb(_, _, _) => self
        }
    }

    #[inline]
    pub fn as_rgba(self) -> Self {
        match self {
            Self::Hex(hex) => {
                let (r, g, b) = (
                    ((hex >> 16) & 0xff) as u8,
                    ((hex >> 8) & 0xff) as u8,
                    (hex & 0xff) as u8
                );
                Self::Rgba(r, g, b, q::as_1())
            },
            Self::Rgb(r, g, b) => Self::Rgba(r, g, b, q::as_1()),
            Self::Rgba(_, _, _, _) => self
        }
    }
}

impl<const A: u8, B, C> From<(u8, u8, u8)> for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: (u8, u8, u8)) -> Self {
        let r: u8 = value.0;
        let g: u8 = value.1;
        let b: u8 = value.2;
        Self::Rgb(r, g, b)
    }
}

impl<const A: u8, B, C> From<u32> for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: u32) -> Self {
        if value > 0xffffff {
            Self::Hex(0xffffff)
        } else {
            Self::Hex(value)
        }
    }
}

impl<const A: u8, B, C> From<u16> for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: u16) -> Self {
        let value: u32 = value.into();
        let value: Self = value.into();
        value
    }
}

impl<const A: u8, B, C> From<u8> for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: u8) -> Self {
        let value: u32 = value.into();
        let value: Self = value.into();
        value
    }
}

impl<const A: u8, B, C, D> From<(u8, u8, u8, D)> for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    D: Into<q::Q<A, B, q::DefaultMode, C>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn from(value: (u8, u8, u8, D)) -> Self {
        let r: u8 = value.0;
        let g: u8 = value.1;
        let b: u8 = value.2;
        let a: q::Q<A, B, q::DefaultMode, C> = value.3.into();
        let a: q::Q<A, B, q::DefaultMode, C> = a.clamp(q::as_0(), q::as_1());
        Self::Rgba(r, g, b, a)
    }
}

impl<const A: u8, B, C> ::core::fmt::Display for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::Hex(code) => write!(f, "#{:06X}", code),
            Self::Rgb(r, g, b) => write!(f, "rgb({}, {}, {})", r, g, b),
            Self::Rgba(r, g, b, a) => write!(f, "rgba({}, {}, {}, {:.3})", r, g, b, a)
        }
    }
}

impl<const A: u8, B, C> Eq for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {}

impl<const A: u8, B, C> PartialEq for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Hex(hex_0), Self::Hex(hex_1)) => {
                hex_0 == hex_1
            },
            (Self::Rgb(r_0, g_0, b_0), Self::Rgb(r_1, g_1, b_1)) => {
                r_0 == r_1 
                && g_0 == g_1 
                && b_0 == b_1
            },
            (Self::Rgba(r_0, g_0, b_0, a_0), Self::Rgba(r_1, g_1, b_1, a_1)) => {
                r_0 == r_1 
                && g_0 == g_1 
                && b_0 == b_1 
                && a_0 == a_1
            },
            (o_0, o_1) => {
                let o_0: Self = o_0.as_rgba();
                let o_1: Self = o_1.as_rgba();
                o_0 == o_1
            }
        }
    }
}

#[test]
fn test_interpolation() {
    let color: Color<2, u32> = (25, 25, 25).into();
    let new_color: Color<2, u32> = color.interpolate((25, 25, 25), 0_25).unwrap();
    assert_eq!(color, new_color);
}