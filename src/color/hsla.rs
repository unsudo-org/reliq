use crate::ops::ToPrim;

use super::*;


// MARK: HslaMode

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct HslaMode<const A: u8 = 1, B = usize>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    h: u16,
    s: q::Q<A, B>,
    l: q::Q<A, B>,
    a: q::Q<A, B>
}

impl<const A: u8, B> Mode for HslaMode<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> 
    {}


// MARK: Hsla

pub type Hsla<const A: u8 = 1, B = usize> = Color<A, B, HslaMode<A, B>>;

impl<const A: u8, B> Hsla<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>  {
    pub fn new(
        h: impl Into<u16>,
        s: impl Into<q::Q<A, B>>,
        l: impl Into<q::Q<A, B>>,
        a: impl Into<q::Q<A, B>>
    ) -> Self {
        let h: u16 = h.into();
        let s: q::Q<A, B> = s.into();
        let l: q::Q<A, B> = l.into();
        let a: q::Q<A, B> = a.into();
        Self {
            mode: HslaMode {
                h,
                s,
                l,
                a
            },
            m_0: ::core::marker::PhantomData
        }
    }

    pub fn h(self) -> u16 {
        self.mode.h
    }

    pub fn s(self) -> q::Q<A, B> {
        self.mode.s
    }

    pub fn l(self) -> q::Q<A, B> {
        self.mode.l
    }

    pub fn a(self) -> q::Q<A, B> {
        self.mode.a
    }

    pub fn complement(self) -> Result<Self> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };      
        let h: u16 = self.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h: q::Q<A, B> = (h + n(180_0)?)?;
        let h: q::Q<A, B> = (h % n(360_0)?)?;
        let h: u16 = h.to_u16()?;
        let s: q::Q<A, B> = self.s();
        let l: q::Q<A, B> = self.l();
        let a: q::Q<A, B> = self.a();
        let ret: Self = Self::new(h, s, l, a);
        Ok(ret)
    }

    pub fn triadic(&self) -> Result<[Self; 3]> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let h: u16 = self.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h_0: q::Q<A, B> = h;
        let h_0: u16 = h_0.to_u16()?;
        let h_1: q::Q<A, B> = (h + n(120_0)?)?;
        let h_1: q::Q<A, B> = (h_1 % n(360_0)?)?;
        let h_1: u16 = h_1.to_u16()?; 
        let h_2: q::Q<A, B> = (h + n(120_0)?)?;
        let h_2: q::Q<A, B> = (h_2 + n(120_0)?)?;
        let h_2: q::Q<A, B> = (h_2 % n(360_0)?)?;
        let h_2: u16 = h_2.to_u16()?;
        let s: q::Q<A, B> = self.s();
        let l: q::Q<A, B> = self.l();
        let a: q::Q<A, B> = self.a();
        let ret: [Self; 3] = [
            Self::new(h_0, s, l, a).into(),
            Self::new(h_1, s, l, a).into(),
            Self::new(h_2, s, l, a).into()
        ];
        Ok(ret)
    }

    pub fn tetradic(self) -> Result<[Self; 4]> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let h: u16 = self.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h: q::Q<A, B> = (h + n(90_0)?)?;
        let h: q::Q<A, B> = (h % n(360_0)?)?;
        let s: q::Q<A, B> = self.s();
        let l: q::Q<A, B> = self.l();
        let a: q::Q<A, B> = self.a();
        let new_color: _ = |x: q::Q<A, B>| -> Result<Self> {
            let offset: q::Q<A, B> = (n(90_0)? * x)?;
            let h: q::Q<A, B> = (h + offset)?;
            let h: q::Q<A, B> = (h % n(360_0)?)?;
            let h: u16 = h.to_u16()?;
            let ret: Self = Self::new(h, s, l, a);
            Ok(ret)
        };
        let ret: [Self; 4] = [
            new_color(n(0_0)?)?,
            new_color(n(1_0)?)?,
            new_color(n(2_0)?)?,
            new_color(n(3_0)?)?
        ];
        Ok(ret)
    }

    pub fn analogous(self) -> Result<[Self; 12]> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let h: u16 = self.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let s: q::Q<A, B> = self.s();
        let l: q::Q<A, B> = self.l();
        let a: q::Q<A, B> = self.a();
        let new_color: _ = |x: q::Q<A, B>| -> Result<Self> {
            let offset: q::Q<A, B> = (n(300)? * x)?;
            let h: q::Q<A, B> = (h + offset)?;
            let h: q::Q<A, B> = (h % n(3600)?)?;
            let h: u16 = h.to_u16()?;
            let ret: Self = Self::new(h, s, l, a);
            Ok(ret)
        };
        let ret: [Self; 12] = [
            new_color(n(0_0)?)?,
            new_color(n(1_0)?)?,
            new_color(n(2_0)?)?,
            new_color(n(3_0)?)?,
            new_color(n(4_0)?)?,
            new_color(n(5_0)?)?,
            new_color(n(6_0)?)?,
            new_color(n(7_0)?)?,
            new_color(n(8_0)?)?,
            new_color(n(9_0)?)?,
            new_color(n(10_0)?)?,
            new_color(n(11_0)?)?
        ];
        Ok(ret)
    }

    pub fn monochromatic<const C: usize>(self) -> Result<array::Array<C, Self>> {
        if C == 0 {
            let ret: array::Array<C, Self> = array::Array::build().finish().unwrap();
            return Ok(ret)
        }
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let h: u16 = self.h();
        let s: q::Q<A, B> = self.s();
        let a: q::Q<A, B> = self.a();
        let l_start: q::Q<A, B> = n(10_0)?;
        let l_end: q::Q<A, B> = n(90_0)?;
        let range: q::Q<A, B> = (l_end - l_start)?;
        let steps: B = if C > 1 {
            let ret: usize = C - 1;
            let ret: B = ret.try_into().ok().ok_or(Error::UnsupportedConversion)?;
            ret
        } else {
            B::AS_1
        };
        let steps: q::Q<A, B> = steps.into();
        let mut ret: array::Array<C, Self> = array::Array::build().finish().unwrap();
        for key in 0..C {
            let key: B = key.try_into().ok().ok_or(Error::UnsupportedConversion)?;
            let key: q::Q<A, B> = key.into();
            let ratio: q::Q<A, B> = (key / steps)?;
            let l: q::Q<A, B> = (range * ratio)?;
            let l: q::Q<A, B> = (l_start + l)?;
            let color: Self = Self::new(h, s, l, a);
            ret.push(color).unwrap();
        }
        Ok(ret)
    }

    pub fn saturate(self, pct: impl Into<q::Percentage<A, B>>) -> Result<Self> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let pct: q::Percentage<A, B> = pct.into();
        let pct: q::Q<A, B> = pct.into();
        let h: u16 = self.h();
        let s: q::Q<A, B> = self.s();
        let s_diff: q::Q<A, B> = (s + pct)?;
        let s: q::Q<A, B> = if s_diff > n(1_0)? {
            n(1_0)?
        } else {
            s_diff
        };
        let l: q::Q<A, B> = self.l();
        let a: q::Q<A, B> = self.a();
        let ret: Self = Self::new(h, s, l, a);
        Ok(ret)
    }

    pub fn desaturate(self, pct: impl Into<q::Percentage<A, B>>) -> Result<Self> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let pct: q::Percentage<A, B> = pct.into();
        let pct: q::Q<A, B> = pct.into();
        let h: u16 = self.h();
        let s: q::Q<A, B> = self.s();
        let s_diff: q::Q<A, B> = (s - pct)?;
        let s: q::Q<A, B> = if s_diff < n(0_0)? {
            n(0_0)?
        } else {
            s_diff
        };
        let l: q::Q<A, B> = self.l();
        let a: q::Q<A, B> = self.a();
        let ret: Self = Self::new(h, s, l, a);
        Ok(ret)
    }

    pub fn lighten(self, pct: impl Into<q::Percentage<A, B>>) -> Result<Self> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let pct: q::Percentage<A, B> = pct.into();
        let pct: q::Q<A, B> = pct.into();
        let h: u16 = self.h();
        let s: q::Q<A, B> = self.s();
        let l: q::Q<A, B> = self.l();
        let l_diff: q::Q<A, B> = (l + pct)?;
        let l: q::Q<A, B> = if l_diff > n(1_0)? {
            n(1_0)?
        } else {
            l_diff
        };
        let a: q::Q<A, B> = self.a();
        let ret: Self = Self::new(h, s, l, a);
        Ok(ret)
    }

    pub fn darken(self, pct: impl Into<q::Percentage<A, B>>) -> Result<Self> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let pct: q::Percentage<A, B> = pct.into();
        let pct: q::Q<A, B> = pct.into();
        let h: u16 = self.h();
        let s: q::Q<A, B> = self.s();
        let l: q::Q<A, B> = self.l();
        let l_diff: q::Q<A, B> = (l - pct)?;
        let l: q::Q<A, B> = if l_diff < n(0_0)? {
            n(0_0)?
        } else {
            l_diff
        };
        let a: q::Q<A, B> = self.a();
        let ret: Self = Self::new(h, s, l, a);
        Ok(ret)
    }

    pub fn interpolate(
        self, 
        rhs: impl Into<Self>, 
        pct: impl Into<q::Percentage<A, B>>
    ) -> Result<Self> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let lhs: Self = self;
        let rhs: Self = rhs.into();
        let pct: q::Percentage<A, B> = pct.into();
        let pct: q::Q<A, B> = pct.into();
        let h_rhs: u16 = rhs.h();
        let h_rhs: B = h_rhs
            .try_into()
            .ok()
            .ok_or(Error::UnsupportedConversion)?;
        let h_rhs: q::Q<A, B> = h_rhs.into();
        let h_lhs: u16 = lhs.h();
        let h_lhs: B = h_lhs
            .try_into()
            .ok()
            .ok_or(Error::UnsupportedConversion)?;
        let h_lhs: q::Q<A, B> = h_lhs.into();
        let full: q::Q<A, B> = n(360_0)?;
        let half: q::Q<A, B> = n(180_0)?;
        let delta: q::Q<A, B> = (h_rhs + full)?;
        let delta: q::Q<A, B> = (delta - h_lhs)?;
        let delta: q::Q<A, B> = (delta % full)?;
        let h: q::Q<A, B> = if delta > half {
            let offset: q::Q<A, B> = (full - delta)?;
            let offset: q::Q<A, B> = (offset * pct)?;
            let ret: q::Q<A, B> = (h_lhs + full)?;
            let ret: q::Q<A, B> = (ret - offset)?;
            let ret: q::Q<A, B> = (ret % full)?;
            ret
        } else {
            let offset: q::Q<A, B> = (delta * pct)?;
            let ret: q::Q<A, B> = (h_lhs + offset)?;
            let ret: q::Q<A, B> = (ret % full)?;
            ret
        };
        let h: u16 = h.to_u16()?;
        let lerp: _ = |x: q::Q<A, B>, y: q::Q<A, B>| -> Result<q::Q<A, B>> {
            let ret: q::Q<A, B> = (y - x)?;
            let ret: q::Q<A, B> = (ret * pct)?;
            let ret: q::Q<A, B> = (x + ret)?;
            Ok(ret)
        };
        let s_rhs: q::Q<A, B> = rhs.s();
        let s_lhs: q::Q<A, B> = lhs.s();
        let s: q::Q<A, B> = lerp(s_lhs, s_rhs)?;
        let l_rhs: q::Q<A, B> = rhs.l();
        let l_lhs: q::Q<A, B> = lhs.l();
        let l: q::Q<A, B> = lerp(l_lhs, l_rhs)?;
        let a_rhs: q::Q<A, B> = rhs.a();
        let a_lhs: q::Q<A, B> = lhs.a();
        let a: q::Q<A, B> = lerp(a_lhs, a_rhs)?;
        let ret: Self = Self::new(h, s, l, a);
        Ok(ret)
    }
}

impl<
    const A: u8, 
    B, 
    C, 
    D, 
    E, 
    F
> From<(C, D, E, F)> for Hsla<A, B>
where
    B: ops::Int,
    C: Into<u16>,
    D: Into<q::Q<A, B>>,
    E: Into<q::Q<A, B>>,
    F: Into<q::Q<A, B>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: (C, D, E, F)) -> Self {
        let h: u16 = value.0.into();
        let s: q::Q<A, B> = value.1.into();
        let l: q::Q<A, B> = value.2.into();
        let a: q::Q<A, B> = value.3.into();
        Self::new(h, s, l, a)
    }
}

impl<
    const A: u8,
    B,
    C,
    D,
    E
> From<(C, D, E)> for Hsla<A, B>
where
    B: ops::Int,
    C: Into<u16>,
    D: Into<q::Q<A, B>>,
    E: Into<q::Q<A, B>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    fn from(value: (C, D, E)) -> Self {
        let h: u16 = value.0.into();
        let s: q::Q<A, B> = value.1.into();
        let l: q::Q<A, B> = value.2.into();
        let a: B = B::AS_1 * B::AS_10;
        let a: q::Q<1, B> = a.into();
        let a: q::Q<A, B> = a.cast::<A>().unwrap();
        Self::new(h, s, l, a)
    }
}

impl<const A: u8, B> From<Hsl<A, B>> for Hsla<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    fn from(value: Hsl<A, B>) -> Self {
        let h: u16 = value.h();
        let s: q::Q<A, B> = value.s();
        let l: q::Q<A, B> = value.l();
        let a: q::Q<A, B> = q::as_1();
        let ret: Self = Self::new(h, s, l, a);
        ret
    }
}

impl<const A: u8, B> TryFrom<Hex<A, B>> for Hsla<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    type Error = Error;

    #[inline]
    fn try_from(value: Hex<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let code: u32 = *value;
        let r: u32 = code >> 16;
        let r: u32 = r & 0xFF;
        let r: B = r.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let r: q::Q<A, B> = r.into();
        let r: q::Q<A, B> = (r / n(255_0)?)?;
        let g: u32 = code >> 8;
        let g: u32 = g & 0xFF;
        let g: B = g.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let g: q::Q<A, B> = g.into();
        let g: q::Q<A, B> = (g / n(255_0)?)?;
        let b: u32 = code & 0xFF;
        let b: B = b.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let b: q::Q<A, B> = b.into();
        let b: q::Q<A, B> = (b / n(255_0)?)?;
        let max: q::Q<A, B> = g.max(b);
        let max: q::Q<A, B> = r.max(max);
        let min: q::Q<A, B> = g.min(b);
        let min: q::Q<A, B> = r.min(min);
        let delta: q::Q<A, B> = (max - min)?;
        let l: q::Q<A, B> = (max + min)?;
        let l: q::Q<A, B> = (l / n(2_0)?)?;
        let s: q::Q<A, B> = if delta == n(0_0)? {
            n(0_0)?
        } else {
            let ret: q::Q<A, B> = (n(2_0)? * l)?;
            let ret: q::Q<A, B> = (ret - n(1_0)?)?;
            let ret: q::Q<A, B> = (n(1_0)? - ret)?;
            let ret: q::Q<A, B> = (delta / ret)?;
            ret
        };
        let h: q::Q<A, B> = if delta == n(0_0)? {
            n(0_0)?
        } else if max == r {
            let ret: q::Q<A, B> = (g - b)?;
            let ret: q::Q<A, B> = (ret / delta)?;
            let ret: q::Q<A, B> = (ret % n(6_0)?)?;
            let ret: q::Q<A, B> = (n(60_0)? * ret)?;
            ret
        } else if max == g {
            let ret: q::Q<A, B> = (b - r)?;
            let ret: q::Q<A, B> = (ret / delta)?;
            let ret: q::Q<A, B> = (ret + n(2_0)?)?;
            let ret: q::Q<A, B> = (n(60_0)? * ret)?;
            ret
        } else {
            let ret: q::Q<A, B> = (r - g)?;
            let ret: q::Q<A, B> = (ret / delta)?;
            let ret: q::Q<A, B> = (ret + n(4_0)?)?;
            let ret: q::Q<A, B> = (n(60_0)? * ret)?;
            ret
        };
        let h: q::Q<A, B> = if h < n(0_0)? {
            let ret: q::Q<A, B> = (h + n(360_0)?)?;
            ret
        } else {
            h
        };
        let h: u16 = h.to_u16()?;
        let s: q::Q<A, B> = (s * n(100_0)?)?;
        let l: q::Q<A, B> = (l * n(100_0)?)?;
        let ret: Self = (h, s, l).into();
        Ok(ret)
    }
}

impl<const A: u8, B> TryFrom<Rgb<A, B>> for Hsla<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    type Error = Error;

    fn try_from(value: Rgb<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let r: u8 = value.r();
        let r: B = r.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let r: q::Q<A, B> = r.into();
        let g: u8 = value.g();
        let g: B = g.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let g: q::Q<A, B> = g.into();
        let b: u8 = value.b();
        let b: B = b.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let b: q::Q<A, B> = b.into();
        let max: q::Q<A, B> = r.max(g).max(b);
        let min: q::Q<A, B> = r.min(g).min(b);
        let delta: q::Q<A, B> = (max - min)?;
        let l: q::Q<A, B> = (max + min)?;
        let l: q::Q<A, B> = (l / n(2_0)?)?;
        let s: q::Q<A, B> = if delta == n(0_0)? {
            n(0_0)?
        } else {
            let dn: q::Q<A, B> = if l < n(0_5)? {
                let ret: q::Q<A, B> = (max + min)?;
                ret
            } else {
                let ret: q::Q<A, B> = (n(2_0)? - max)?;
                let ret: q::Q<A, B> = (ret - min)?;
                ret
            };
            let ret: q::Q<A, B> = (delta / dn)?;
            ret
        };
        let h: q::Q<A, B> = if delta == n(0_0)? {
            n(0_0)?
        } else if max == r {
            let ret: q::Q<A, B> = (g - b)?;
            let ret: q::Q<A, B> = (ret / delta)?;
            let ret: q::Q<A, B> = (ret % n(6_0)?)?;
            let ret: q::Q<A, B> = (ret * n(60_0)?)?;
            ret
        } else if max == g {
            let ret: q::Q<A, B> = (b - r)?;
            let ret: q::Q<A, B> = (ret / delta)?;
            let ret: q::Q<A, B> = (ret + n(2_0)?)?;
            let ret: q::Q<A, B> = (ret * n(60_0)?)?;
            ret
        } else {
            let ret: q::Q<A, B> = (r - g)?;
            let ret: q::Q<A, B> = (ret / delta)?;
            let ret: q::Q<A, B> = (ret + n(4_0)?)?;
            let ret: q::Q<A, B> = (ret * n(60_0)?)?;
            ret
        };
        let h: q::Q<A, B> = if h < n(0_0)? {
            (h + n(360_0)?)?
        } else {
            h
        };
        let h: u16 = h.to_u16()?;
        let s: q::Q<A, B> = (s * n(100_0)?)?;
        let l: q::Q<A, B> = (l * n(100_0)?)?;
        let ret: Self = (h, s, l).into();
        Ok(ret)
    }
}

impl<const A: u8, B> TryFrom<Rgba<A, B>> for Hsla<A, B>
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
        let r: u8 = value.r();
        let r: B = r.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let r: q::Q<A, B> = r.into();
        let g: u8 = value.g();
        let g: B = g.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let g: q::Q<A, B> = g.into();
        let b: u8 = value.b();
        let b: B = b.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let b: q::Q<A, B> = b.into();
        let a: q::Q<A, B> = value.a();
        let max: q::Q<A, B> = r.max(g).max(b);
        let min: q::Q<A, B> = r.min(g).min(b);
        let delta: q::Q<A, B> = (max - min)?;
        let l: q::Q<A, B> = (max + min)?;
        let l: q::Q<A, B> = (l / n(2_0)?)?;
        let s: q::Q<A, B> = if delta == n(0_0)? {
            n(0_0)?
        } else {
            let dn: q::Q<A, B> = if l < n(0_5)? {
                (max + min)?
            } else {
                let ret: q::Q<A, B> = (n(2_0)? - max)?;
                let ret: q::Q<A, B> = (ret - min)?;
                ret
            };
            let ret: q::Q<A, B> = (delta / dn)?;
            ret
        };
        let h: q::Q<A, B> = if delta == n(0_0)? {
            n(0_0)?
        } else if max == r {
            let ret: q::Q<A, B> = (g - b)?;
            let ret: q::Q<A, B> = (ret / delta)?;
            let ret: q::Q<A, B> = (ret % n(6_0)?)?;
            let ret: q::Q<A, B> = (ret * n(60_0)?)?;
            ret
        } else if max == g {
            let ret: q::Q<A, B> = (b - r)?;
            let ret: q::Q<A, B> = (ret / delta)?;
            let ret: q::Q<A, B> = (ret + n(2_0)?)?;
            let ret: q::Q<A, B> = (ret * n(60_0)?)?;
            ret
        } else {
            let ret: q::Q<A, B> = (r - g)?;
            let ret: q::Q<A, B> = (ret / delta)?;
            let ret: q::Q<A, B> = (ret + n(4_0)?)?;
            let ret: q::Q<A, B> = (ret * n(60_0)?)?;
            ret
        };
        let h: q::Q<A, B> = if h < n(0_0)? {
            let ret: q::Q<A, B> = (h + n(360_0)?)?;
            ret
        } else {
            h
        };
        let h: u16 = h.to_u16()?;
        let s: q::Q<A, B> = (s * n(100_0)?)?;
        let l: q::Q<A, B> = (l * n(100_0)?)?;
        let ret: Self = (h, s, l, a).into();
        Ok(ret)
    }
}

impl<const A: u8, B> CommonAlphaExt<A, B> for Hsla<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> 
    {}