use super::*;

pub type Rgb<const A: u8, B = usize> = Color<A, B, RgbMode>;

#[derive(Debug)]
#[derive(Clone)]
pub struct RgbMode {
    r: u8,
    g: u8,
    b: u8
}

impl Mode for RgbMode {}

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
        let t: q::Unit<A, B, C> = t.into();
        let r_0: q::Unit<A, B, C> = q::r#as::<0, _, u8, _, _, _, _>(self.mode.r)?;
        let g_0: q::Unit<A, B, C> = q::r#as::<0, _, u8, _, _, _, _>(self.mode.g)?;
        let b_0: q::Unit<A, B, C> = q::r#as::<0, _, u8, _, _, _, _>(self.mode.b)?;
        let r_1: q::Unit<A, B, C> = q::r#as::<0, _, u8, _, _, _, _>(rhs.mode.r)?;
        let g_1: q::Unit<A, B, C> = q::r#as::<0, _, u8, _, _, _, _>(rhs.mode.g)?;
        let b_1: q::Unit<A, B, C> = q::r#as::<0, _, u8, _, _, _, _>(rhs.mode.b)?;
        let r: q::Unit<A, B, C> = r_0.lerp(r_1, t)?;
        let r: u8 = r.to_u8()?;
        let g: q::Unit<A, B, C> = g_0.lerp(g_1, t)?;
        let g: u8 = g.to_u8()?;
        let b: q::Unit<A, B, C> = b_0.lerp(b_1, t)?;
        let b: u8 = b.to_u8()?;
        Ok(Self {
            mode: RgbMode {
                r,
                g,
                b
            },
            n: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        })
    }
}

impl<const A: q::Precision, B, C, D> From<Hex<A, B, C, D>> for Rgb<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: Hex<A, B, C, D>) -> Self {
        let hex: Hex<A, B, C, D> = value;
        let r: u8 = ((*hex >> 16) & 0xFF) as u8;
        let g: u8 = ((*hex >> 8) & 0xFF) as u8;
        let b: u8 = (*hex & 0xFF) as u8;
        Self {
            mode: RgbMode {
                r,
                g,
                b
            },
            n: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        }
    }
}

impl<const A: q::Precision, B, C, D> TryFrom<Hsl<A, B, C, D>> for Rgb<A, B, C, D>
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
        use ops::ToPrim as _;
        let hsl: Hsl<A, B, C, D> = value;
        let h: u16 = *hsl.h();
        let h: u16 = h.clamp(0, 360);
        let h: q::Unit<A, B, C> = q::r#as::<0, _, u16, _, _, _, _>(h)?;
        let h: q::Unit<A, B, C> = (h / q::r#as::<0, _, u16, _, _, _, _>(360_u16)?)?;
        let s: q::Unit<A, B, C> = *hsl.s();
        let s: q::Unit<A, B, C> = s.clamp(q::as_0::<A, B, _, C>(), q::as_100::<A, B, _, C>());
        let s: q::Unit<A, B, C> = (s / q::r#as::<0, _, u16, _, _, _, _>(100_u16)?)?;
        let l: q::Unit<A, B, C> = *hsl.l();
        let l: q::Unit<A, B, C> = l.clamp(q::as_0::<A, B, _, C>(), q::as_100::<A, B, _, C>());
        let l: q::Unit<A, B, C> = (l / q::r#as::<0, _, u16, _, _, _, _>(100_u16)?)?;
        let (r, g, b) = if s == 0 {
            let gray: u8 = (l * q::r#as::<0, _, u8, _, _, _, _>(255_u8)?)?.round_down().to_u8()?;
            (gray, gray, gray)
        } else {
            let q = if l < (q::as_1::<A, B, _, C>() / q::as_2::<A, B, _, C>())? {
                (l * (q::as_1() + s)?)?
            } else {
                let a = (l * s)?;
                let b = (l + s)?;
                let c = (a - b)?;
                c
            };
        };
    }
}