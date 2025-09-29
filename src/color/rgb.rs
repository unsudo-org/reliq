use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct RgbMode {
    r: u8,
    g: u8,
    b: u8
}

impl Mode for RgbMode {}


pub type Rgb<const A: u8, B = usize> = Color<A, B, RgbMode>;

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

    #[inline]
    pub fn interpolate<C, D>(self, rhs: C, t: D) -> Result<Self>
    where
        C: Into<Self>,
        D: Into<q::Unit<A, B>> {
        use ops::ToPrim as _;
        let rhs: Self = rhs.into();
        let t: q::Unit<A, B> = t.into();
        let r_0: q::Unit<A, B> = q::r#as::<0, _, u8, _, _, _>(self.mode.r)?;
        let g_0: q::Unit<A, B> = q::r#as::<0, _, u8, _, _, _>(self.mode.g)?;
        let b_0: q::Unit<A, B> = q::r#as::<0, _, u8, _, _, _>(self.mode.b)?;
        let r_1: q::Unit<A, B> = q::r#as::<0, _, u8, _, _, _>(rhs.mode.r)?;
        let g_1: q::Unit<A, B> = q::r#as::<0, _, u8, _, _, _>(rhs.mode.g)?;
        let b_1: q::Unit<A, B> = q::r#as::<0, _, u8, _, _, _>(rhs.mode.b)?;
        let r: q::Unit<A, B> = r_0.lerp(r_1, t)?;
        let r: u8 = r.to_u8()?;
        let g: q::Unit<A, B> = g_0.lerp(g_1, t)?;
        let g: u8 = g.to_u8()?;
        let b: q::Unit<A, B> = b_0.lerp(b_1, t)?;
        let b: u8 = b.to_u8()?;
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