use crate::{ops::ToPrim, q::r#as};

use super::*;

pub struct HslMode<const A: u8, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub(super) h: u16,
    pub(super) s: q::Unit<A, B>,
    pub(super) l: q::Unit<A, B>
}

impl<const A: u8, B> Mode for HslMode<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>
    {}


pub type Hsl<const A: u8 = 1, B = usize> = Color<A, B, HslMode<A, B>>;

impl<const A: u8, B> Hsl<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub fn h(&self) -> u16 {
        self.mode.h
    }

    pub fn s(&self) -> q::Unit<A, B> {
        self.mode.s
    }

    pub fn l(&self) -> q::Unit<A, B> {
        self.mode.l
    }

    pub fn complement(self) -> Result<Self> {
        let n180: q::Q<A, B> = r#as::<1, _, u32, _, _, u32>(1800)?;
        let n360: q::Q<A, B> = r#as::<1, _, u32, _, _, u32>(3600)?;
        let h: u16 = self.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h: q::Q<A, B> = (h + n180)?;
        let h: q::Q<A, B> = (h % n360)?;
        let h: u16 = h.to_u16()?;
        let s: q::Q<A, B> = self.s();
        let l: q::Q<A, B> = self.l();
        let ret: Self = Self {
            mode: HslMode {
                h,
                s,
                l
            },
            m_0: ::core::marker::PhantomData
        };
        Ok(ret)
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
}

impl<const A: u8, B> From<Hex<A, B>> for Hsl<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: Hex<A, B>) -> Self {
        todo!()
    }
}

impl<const A: u8, B> From<Hsla<A, B>> for Hsl<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: Hsla<A, B>) -> Self {
        todo!()
    }
}

impl<const A: u8, B> TryFrom<Rgba<A, B>> for Hsl<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Error = Error;

    #[inline]
    fn try_from(value: Rgba<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let ret: Rgb<A, B> = value.into();
        let ret: Self = ret.try_into()?;
        Ok(ret)
    }
}

impl<const A: u8, B> TryFrom<Rgb<A, B>> for Hsl<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    type Error = Error;

    #[allow(clippy::inconsistent_digit_grouping)]
    #[inline]
    fn try_from(value: Rgb<A, B>) -> ::core::result::Result<Self, Self::Error> {
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
        let dt: q::Delta<A, B> = (max - min)?.into();
        let as_0: q::Q<A, B> = q::as_0();
        let as_1: q::Q<A, B> = q::as_1();
        let as_2: q::Q<A, B> = q::as_2();
        let as_4: q::Q<A, B> = q::as_4();
        let as_6: q::Q<A, B> = q::as_6();
        let as_60: q::Q<A, B> = q::as_60();
        let l: q::Q<A, B> = (max + min)?;
        let l: q::Q<A, B> = (l / as_2)?;
        let s: q::Q<A, B> = if dt == as_0 {
            as_0
        } else {
            let ret: q::Q<A, B> = (as_0 * l)?;
            let ret: q::Q<A, B> = (ret - as_1)?;
            let ret: q::Q<A, B> = (as_1 - ret)?;
            let ret: q::Q<A, B> = (dt / ret)?.into();
            ret 
        };
        let h: q::Q<A, B> = if dt == as_0 {
            as_0
        } else if max == r {
            let ret: q::Q<A, B> = (g - b)?;
            let ret: q::Q<A, B> = (ret / dt)?;
            let ret: q::Q<A, B> = (ret % as_6)?;
            let ret: q::Q<A, B> = (as_60 * ret)?;
            ret
        } else if max == g {
            let ret: q::Q<A, B> = (b - r)?;
            let ret: q::Q<A, B> = (ret / dt)?;
            let ret: q::Q<A, B> = (ret + as_2)?;
            let ret: q::Q<A, B> = (as_60 * ret)?;
            ret
        } else {
            let ret: q::Q<A, B> = (r - g)?;
            let ret: q::Q<A, B> = (ret / dt)?;
            let ret: q::Q<A, B> = (ret + as_4)?;
            let ret: q::Q<A, B> = (as_60 * ret)?;
            ret
        };
        let h: q::Q<A, B> = (h + q::r#as::<1, _, u64, _, q::UnitMode, _>(360_0_u64))?;
        let h: q::Q<A, B> = (h % q::r#as::<1, _, u64, _, q::UnitMode, _>(360_0_u64))?;
        let h: q::Q<A, B> = h.round_down();
        let h: u16 = h.to_u16()?;
        let ret: Self = Self {
            mode: HslMode {
                h,
                s,
                l
            },
            m_0: ::core::marker::PhantomData
        };
        Ok(ret)
    }
}

impl<const A: u8, B> CommonAlphaExt<A, B> for Hsl<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>
    {}