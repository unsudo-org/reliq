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
        let ret: Self = self.normalize();
        matches!(ret, Self::Hex(_))
    }

    #[inline]
    pub fn is_rgb(self) -> bool {
        let ret: Self = self.normalize();
        matches!(ret, Self::Rgb(_, _, _))
    }

    #[inline]
    pub fn is_rgba(self) -> bool {
        let ret: Self = self.normalize();
        matches!(ret, Self::Rgba(_, _, _, _))
    }

    #[inline]
    pub fn interpolate<D, E>(self, rhs: D, t: E) -> Result<Self>
    where
        D: Into<Self>,
        E: Into<q::Q<A, B, q::DefaultMode, C>> {
        let x: Color<A, B, C> = self.normalize();
        let y: Self = rhs.into();
        let y: Color<A, B, C> = y.normalize();
        let t: q::Q<A, B, q::DefaultMode, C> = t.into();
        let t: q::Q<A, B, q::DefaultMode, C> = t.clamp(q::as_0(), q::as_1());
        match (x, y) {
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
        match self.normalize() {
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
        match self.normalize() {
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
        match self.normalize() {
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

    #[inline]
    fn normalize(self) -> Self {
        match self {
            Self::Hex(code) => {
                let ret: u32 = code.clamp(0x000000, 0xFFFFFF);
                let ret: Self = Self::Hex(ret);
                ret
            },
            Self::Rgb(_, _, _) => self,
            Self::Rgba(r, g, b, a) => {
                let ret: q::Q<A, B, q::DefaultMode, C> = a.clamp(q::as_0(), q::as_1());
                let ret: Self = Self::Rgba(r, g, b, ret);
                ret
            }
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
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn from(value: (u8, u8, u8)) -> Self {
        let r: u8 = value.0;
        let g: u8 = value.1;
        let b: u8 = value.2;
        Self::Rgb(r, g, b).normalize()
    }
}

impl<const A: u8, B, C> From<u32> for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn from(value: u32) -> Self {
        Self::Hex(value).normalize()
    }
}

impl<const A: u8, B, C> From<u16> for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
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
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
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
        Self::Rgba(r, g, b, a).normalize()
    }
}

impl<const A: u8, B, C> ::core::fmt::Display for Color<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self.normalize() {
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
        let x: Color<A, B, C> = self.normalize();
        let y: Color<A, B, C> = other.normalize();
        match (x, y) {
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

#[cfg(test)]
#[::rstest::rstest]
#[case((25, 25, 25), (50, 50, 50), (25, 25, 25))]
#[case((0, 0, 0), (255, 255, 255), (127, 127, 127))]
#[case((100, 150, 200), (50, 75, 100), (75, 112, 150))]
#[case((10, 20, 30), (240, 220, 200), (125, 120, 115))]
#[case((255, 0, 0), (0, 255, 0), (128, 128, 0))]
#[case((255, 255, 255), (0, 0, 0), (128, 128, 128))]
#[case((10, 50, 100), (100, 150, 200), (55, 100, 150))]
fn test_interpolation<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C) 
where
    A: Into<Color<2, i128>>,
    B: Into<Color<2, i128>>,
    C: Into<Color<2, i128>> {
    let x: Color<2, i128> = x.into();
    let y: Color<2, i128> = y.into();
    let ok: Color<2, i128> = ok.into();
    let ret: Color<2, i128> = x.interpolate(y, 0_50).unwrap();
    assert_eq!(ret, ok);
}