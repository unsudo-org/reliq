impl<const A: u8, B> Color<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {

    #[inline]
    pub fn lighten<C>(self, multiplier: C) -> Result<Self> 
    where
        C: Into<q::Unit<A, B>> {
        let multiplier: q::Unit<A, B> = multiplier.into();
        self.interpolate::<(u8, u8, u8), q::Unit<A, B>>((255, 255, 255), multiplier)
    }

    #[inline]
    pub fn darken<D>(self, multiplier: D) -> Result<Self> 
    where
        D: Into<q::Unit<A, B>> {
        let multiplier: q::Unit<A, B> = multiplier.into();
        self.interpolate::<(u8, u8, u8), q::Unit<A, B>>((0, 0, 0), multiplier)
    }

    #[inline]
    pub fn saturate<D>(self, percentage: q::Percentage<A, B, C>) -> Result<Self> 
    where
        D: Into<q::Q<A, B, q::UnitMode, C>> {

    }

    #[inline]
    pub fn desaturate<D>(self, percentage: q::Percentage<A, B, C>) -> Result<Self> 
    where
        D: Into<q::Q<A, B, q::UnitMode, C>> {
        let multiplier: q::Q<A, B, q::UnitMode, C> = percentage.into();
        match self.to_hsl().anyhow() {
            Self::Hsl(h, s, l) => {
                let s: q::Q<A, B, q::UnitMode, C> = (s * (q::as_1() - multiplier)?)?;
                let s: q::Q<A, B, q::UnitMode, C> = s.clamp(q::as_0(), q::as_1());
                Ok(Self::Hsl(h, s, l))
            },
            Self::Hsla(h, s, l, a) => {
                let s = (s * (q::as_1() - multiplier)?)?;
                
            },
            _ => {
                self.to_hsl().anyhow().desaturate()
            }
        }
    }

    #[inline]
    pub fn complement(self) -> Result<Self> {
        match self.to_hsl().anyhow() {
            Self::Hsl(h, s, l) => {
                let new_h: u16 = (h + 180) % 360;
                Ok(Self::Hsl(new_h, s, l))
            },
            Self::Hsla(h, s, l, a) => {
                let new_h: u16 = (h + 180) % 360;
                Ok(Self::Hsla(new_h, s, l, a))
            },
            _ => {
                self.to_hsl().anyhow().complement()
            }
        }
    }

    pub fn triadic(self) -> Result<[Self; 3]> {
        match self.to_hsl().anyhow() {
            Self::Hsl(h, s, l) => {
                let h_0: u16 = (h + 120) % 360;
                let h_1: u16 = (h + 240) % 360;
                Ok([
                    self,
                    Self::Hsl(h_0, s, l),
                    Self::Hsl(h_1, s, l)
                ])
            },

        }
    }

    pub fn tetradic(self) {

    }

    pub fn analogous(self) {

    }

    pub fn monochromatic<const D: usize>(self) -> array::Array<D, Self> {

    }

    #[inline]
    pub fn interpolate<D, E>(self, rhs: D, t: E) -> Result<Self>
    where
        D: Into<Self>,
        E: Into<q::Unit<A, B, C>> {
        let x: Color<A, B, C> = self.normalize().anyhow();
        let y: Self = rhs.into();
        let y: Color<A, B, C> = y.normalize().anyhow();
        let t: q::Q<A, B, q::UnitMode, C> = t.into();
        let t: q::Q<A, B, q::UnitMode, C> = t.clamp(q::as_0(), q::as_1());
        match (x, y) {
            (Self::Rgb((r_0, g_0, b_0)), Self::Rgb((r_1, g_1, b_1))) => {
                let lerp: _ = |a: u8, b: u8| -> q::Result<u8> {
                    let a: B = a
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedConversion)?;
                    let a: q::Q<A, B, q::UnitMode, C> = a.into();
                    let b: B = b
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedConversion)?;
                    let b: q::Q<A, B, q::UnitMode, C> = b.into();
                    let ret: q::Q<A, B, q::UnitMode, C> = ((a + (b - a)?)? * t)?;
                    let ret: B = ret.into_int();
                    let ret: u8 = ret
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedConversion)?;
                    Ok(ret)
                };
                let r: u8 = lerp(r_0, r_1)?;
                let g: u8 = lerp(g_0, g_1)?;
                let b: u8 = lerp(b_0, b_1)?;
                Ok(Self::Rgb((r, g, b)))
            },
            (Self::Rgba((r_0, g_0, b_0, a_0)), Self::Rgba((r_1, g_1, b_1, a_1))) => {
                let lerp: _ = |a: u8, b: u8| -> q::Result<u8> {
                    let a: B = a
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedConversion)?;
                    let a: q::Q::<A, B, q::UnitMode, C> = a.into();
                    let b: B = b
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedConversion)?;
                    let b: q::Q::<A, B, q::UnitMode, C> = b.into();
                    let ret: q::Q<A, B, q::UnitMode, C> = ((a + (b - a)?)? * t)?;
                    let ret: B = ret.into_int();
                    let ret: u8 = ret
                        .try_into()
                        .ok()
                        .ok_or(q::Error::UnsupportedConversion)?;
                    Ok(ret)
                };
                let a: q::Q<A, B, q::UnitMode, C> = {
                    let ret: q::Q<A, B, q::UnitMode, C> = ((a_0 + (a_1 - a_0)?)? * t)?;
                    let ret: q::Q<A, B, q::UnitMode, C> = ret.clamp(q::as_0(), q::as_1());
                    ret
                };
                let r: u8 = lerp(r_0, r_1)?;
                let g: u8 = lerp(g_0, g_1)?;
                let b: u8 = lerp(b_0, b_1)?;
                Ok(Self::Rgba((r, g, b, a)))
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
                    Self::Rgb((r, g, b)) => Self::Rgba((r, g, b, q::as_1())),
                    o => o
                };
                let rhs: Self = match rhs {
                    Self::Rgb((r, g, b)) => Self::Rgba((r, g, b, q::as_1())),
                    o => o
                };
                let ret: Self = lhs.interpolate(rhs, t)?;
                Ok(ret)
            }
        }
    }

    #[inline]
    pub fn to_hex(self) -> lossy::Lossy<Self> {
        match self.normalize().anyhow() {
            Self::Rgb((r, g, b)) => {
                let ret: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                let ret: Self = Self::Hex(ret);
                lossy::Lossy::Exact(ret)
            },
            Self::Rgba((r, g, b, _)) => {
                let ret: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                let ret: Self = Self::Hex(ret);
                lossy::Lossy::Trunc(ret)
            },
            Self::Hex(_) => lossy::Lossy::Exact(self),
            Self::Hsl((h, s, l)) => {

            },
            Self::Hsla((h, s, l, a)) => {

            }
        }
    }

    #[inline]
    pub fn to_rgb(self) -> lossy::Lossy<Self> {
        match self.normalize().anyhow() {
            Self::Hex(hex) => {
                let (r, g, b) = (
                    ((hex >> 16) & 0xFF) as u8,
                    ((hex >> 8) & 0xFF) as u8,
                    (hex & 0xff) as u8
                );
                
                Self::Rgb(r, g, b)
            },
            Self::Hsl(, , ) => ,
            Self::Rgb(_, _, _) => Exact(self),
            Self::Rgba(r, g, b, _) => Trunc(Self::Rgb(r, g, b)),
            Self::Hsla(, , , )
        }
    }

    #[inline]
    pub fn to_rgba(self) -> Self {
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
    pub fn to_hsl(self) -> Lossy<Self> {
        match self.normalize().anyhow() {
            Self::Hex(code) => {
                let (r, g, b) = (
                    ((code >> 16) & 0xFF) as u8,
                    ((code >> 8) & 0xFF) as u8,
                    (code & 0xFF) as u8
                );
                let (h, s, l) = Self::rgb_to_hsl(r, g, b);
                Self::Hsl(h, s, l)
            },
            Self::Rgb(r, g, b) | Self::Rgba(r, g, b, _) => {
                
            }
        }
    }

    pub fn to_hsla(self) -> Self {

    }

    #[inline]
    fn normalize(self) -> lossy::Lossy<Self> {
        match self {
            Self::Hex(code) => {
                let ret: u32 = code.clamp(0x000000, 0xFFFFFF);
                let ret: Self = Self::Hex(ret);
                lossy::Lossy::Trunc(ret)
            },
            Self::Rgb(_, _, _) => lossy::Lossy::Exact(self),
            Self::Rgba(r, g, b, a) => {
                let ret: q::Q<A, B, q::UnitMode, C> = a.clamp(q::as_0(), q::as_1());
                let ret: Self = Self::Rgba(r, g, b, ret);
                lossy::Lossy::Trunc(ret)
            }
        }
    }

    fn hex_to_hsl(hex: Hex) -> Result<Hsl<A, B, C>> {

    }

    fn hex_to_rgb(hex: Hex) -> Result<Rgb> {

    }

    fn hex_to_rgba(hex: Hex) -> Result<Rgba<A, B, C>> {

    }

    fn rgb_to_hsl(rgb: Rgb) -> Result<Hsl<A, B, C>> {
        let (r, g, b) = rgb;
        let n255: B = 255.try_into()?;
        let n255: q::Unit<A, B, C> = n255.into();
        let rf: B = r
            .try_into()
            .ok()
            .ok_or(Error::UnsupportedConversion)?;
        let rf: q::Unit<A, B, C> = rf.into();
        let rf: q::Unit<A, B, C> = (rf / n255)?;
        let gf: B = g
            .try_into()
            .ok()
            .ok_or(Error::UnsupportedConversion)?;
        let gf: q::Unit<A, B, C> = gf.into();
        let gf: q::Unit<A, B, C> = (gf / n255)?;
        let bf: B = b
            .try_into()
            .ok()
            .ok_or(Error::UnsupportedConversion)?;
        let bf: q::Unit<A, B, C> = bf.into();
        let bf: q::Unit<A, B, C> = (bf / n255)?;
        let max: q::Unit<A, B, C> = rf.max(gf).max(bf);
        let min: q::Unit<A, B, C> = rf.min(gf).min(bf);
        let delta: q::Delta<A, B, C> = (max - min)?.into();
        let l: q::Unit<A, B, C> = (max + min)?;
        let l: q::Unit<A, B, C>  = (l / q::as_2())?;
        let s = if delta == 0.0 {
            
        } else {

        };
    }
}

impl<const A: u8, B, C> From<(u8, u8, u8)> for Color<A, B, C>
where
    B: ops::Int,
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
        Self::Rgb(r, g, b).normalize().anyhow()
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
        Self::Hex(value).normalize().anyhow()
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
    D: Into<q::Q<A, B, q::UnitMode, C>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn from(value: (u8, u8, u8, D)) -> Self {
        let r: u8 = value.0;
        let g: u8 = value.1;
        let b: u8 = value.2;
        let a: q::Q<A, B, q::UnitMode, C> = value.3.into();
        Self::Rgba((r, g, b, a)).normalize().anyhow()
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
        match self.normalize().anyhow() {
            Self::Hex(code) => write!(f, "#{:06X}", code),
            Self::Rgb((r, g, b)) => write!(f, "rgb({}, {}, {})", r, g, b),
            Self::Rgba((r, g, b, a)) => write!(f, "rgba({}, {}, {}, {:.3})", r, g, b, a)
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
        let x: Color<A, B, C> = self.normalize().anyhow();
        let y: Color<A, B, C> = other.normalize().anyhow();
        match (x, y) {
            (Self::Hex(hex_0), Self::Hex(hex_1)) => {
                hex_0 == hex_1
            },
            (Self::Rgb((r_0, g_0, b_0)), Self::Rgb((r_1, g_1, b_1))) => {
                r_0 == r_1 
                && g_0 == g_1 
                && b_0 == b_1
            },
            (Self::Rgba((r_0, g_0, b_0, a_0)), Self::Rgba((r_1, g_1, b_1, a_1))) => {
                r_0 == r_1 
                && g_0 == g_1 
                && b_0 == b_1
                && a_0 == a_1
            },
            (o_0, o_1) => {
                let o_0: Self = o_0.to_rgba();
                let o_1: Self = o_1.to_rgba();
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