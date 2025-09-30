use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct RgbMode {
    r: u8,
    g: u8,
    b: u8
}

impl Mode for RgbMode {}


pub type Rgb<const A: u8 = 1, B = usize> = Color<A, B, RgbMode>;

impl<const A: u8, B> Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
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
        use ops::ToPrim as _;
        let rhs: Self = rhs.into();
        let min: q::Q<A, B> = q::as_0::<A, B, _>();
        let max: q::Q<A, B> = q::as_100::<A, B, _>();
        let percentage: q::Percentage<A, B> = percentage.into();
        let percentage: q::Q<A, B> = percentage.into();
        let percentage: q::Q<A, B> = percentage.clamp(min, max);
        let r_0: q::Q<A, B> = q::r#as::<1, _, u8, _, _, _>(self.mode.r * 10)?;
        let g_0: q::Q<A, B> = q::r#as::<1, _, u8, _, _, _>(self.mode.g * 10)?;
        let b_0: q::Q<A, B> = q::r#as::<1, _, u8, _, _, _>(self.mode.b * 10)?;
        let r_1: q::Q<A, B> = q::r#as::<1, _, u8, _, _, _>(rhs.mode.r * 10)?;
        let g_1: q::Q<A, B> = q::r#as::<1, _, u8, _, _, _>(rhs.mode.g * 10)?;
        let b_1: q::Q<A, B> = q::r#as::<1, _, u8, _, _, _>(rhs.mode.b * 10)?;
        let r: u8 = r_0.lerp(r_1, percentage)?.to_u8()?;
        let g: u8 = g_0.lerp(g_1, percentage)?.to_u8()?;
        let b: u8 = b_0.lerp(b_1, percentage)?.to_u8()?;
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

#[cfg(test)]
mod test {
    use super::*;

    fn interpolate() {
        let x: Rgb = (200, 200, 200).into();
        let y: Rgb = (25, 25, 25).into();
        let z: Rgb = x.interpolate(y, 50_0).unwrap();
        assert_eq!(z, (100, 100, 100).into());
    }
}