use super::*;

::modwire::expose!(
    pub hex
    pub hsl
    pub hsla
    pub mode
    pub rgb
    pub rgba
);

#[allow(clippy::inconsistent_digit_grouping)]
pub trait CommonExt<const A: u8, B> 
where
    Self: TryFrom<Hsla<A, B>>,
    Self: TryInto<Hsla<A, B>>,
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn complement(self) -> Result<Self> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };      
        let hsla: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: u16 = hsla.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h: q::Q<A, B> = (h + n(180_0)?)?;
        let h: q::Q<A, B> = (h % n(360_0)?)?;
        let h: u16 = h.to_u16()?;
        let s: q::Q<A, B> = hsla.s();
        let l: q::Q<A, B> = hsla.l();
        let a: q::Q<A, B> = hsla.a();
        let ret: Hsla<A, B> = Hsla::new(h, s, l, a);
        let ret: Self = ret.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn triadic(self) -> Result<[Self; 3]> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: u16 = hsla.h();
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
        let s: q::Q<A, B> = hsla.s();
        let l: q::Q<A, B> = hsla.l();
        let a: q::Q<A, B> = hsla.a();
        let ret: [Self; 3] = [
            Hsla::new(h_0, s, l, a).try_into().ok().ok_or(Error::UnsupportedConversion)?,
            Hsla::new(h_1, s, l, a).try_into().ok().ok_or(Error::UnsupportedConversion)?,
            Hsla::new(h_2, s, l, a).try_into().ok().ok_or(Error::UnsupportedConversion)?
        ];
        Ok(ret)
    }

    fn tetradic(self) -> Result<[Self; 4]> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: u16 = hsla.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h: q::Q<A, B> = (h + n(90_0)?)?;
        let h: q::Q<A, B> = (h % n(360_0)?)?;
        let s: q::Q<A, B> = hsla.s();
        let l: q::Q<A, B> = hsla.l();
        let a: q::Q<A, B> = hsla.a();
        let new_color: _ = |x: q::Q<A, B>| -> Result<Self> {
            let offset: q::Q<A, B> = (n(90_0)? * x)?;
            let h: q::Q<A, B> = (h + offset)?;
            let h: q::Q<A, B> = (h % n(360_0)?)?;
            let h: u16 = h.to_u16()?;
            let ret: Hsla<A, B> = Hsla::new(h, s, l, a);
            let ret: Self = ret.try_into().ok().ok_or(Error::UnsupportedConversion)?;
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

    fn analogous(self) -> Result<[Self; 12]> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: u16 = hsla.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let s: q::Q<A, B> = hsla.s();
        let l: q::Q<A, B> = hsla.l();
        let a: q::Q<A, B> = hsla.a();
        let new_color: _ = |x: q::Q<A, B>| -> Result<Self> {
            let offset: q::Q<A, B> = (n(300)? * x)?;
            let h: q::Q<A, B> = (h + offset)?;
            let h: q::Q<A, B> = (h % n(3600)?)?;
            let h: u16 = h.to_u16()?;
            let ret: Hsla<A, B> = Hsla::new(h, s, l, a);
            let ret: Self = ret.try_into().ok().ok_or(Error::UnsupportedConversion)?;
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

    fn monochromatic<const C: usize>(self) -> Result<array::Array<C, Self>> {
        if C == 0 {
            let ret: array::Array<C, Self> = array::Array::build().finish().unwrap();
            return Ok(ret)
        }
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: u16 = hsla.h();
        let s: q::Q<A, B> = hsla.s();
        let a: q::Q<A, B> = hsla.a();
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
            let color: Self = Hsla::new(h, s, l, a).try_into().ok().ok_or(Error::UnsupportedConversion)?;
            ret.push(color).unwrap();
        }
        Ok(ret)
    }

    fn saturate<C>(self, pct: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let pct: q::Percentage<A, B> = pct.into();
        let pct: q::Q<A, B> = pct.into();
        let h: u16 = hsla.h();
        let s: q::Q<A, B> = hsla.s();
        let s_diff: q::Q<A, B> = (s + pct)?;
        let s: q::Q<A, B> = if s_diff > n(1_0)? {
            n(1_0)?
        } else {
            s_diff
        };
        let l: q::Q<A, B> = hsla.l();
        let a: q::Q<A, B> = hsla.a();
        let ret: Self = Hsla::new(h, s, l, a).try_into().ok().ok_or(Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn desaturate<C>(self, pct: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let pct: q::Percentage<A, B> = pct.into();
        let pct: q::Q<A, B> = pct.into();
        let h: u16 = hsla.h();
        let s: q::Q<A, B> = hsla.s();
        let s_diff: q::Q<A, B> = (s - pct)?;
        let s: q::Q<A, B> = if s_diff < n(0_0)? {
            n(0_0)?
        } else {
            s_diff
        };
        let l: q::Q<A, B> = hsla.l();
        let a: q::Q<A, B> = hsla.a();
        let ret: Self = Hsla::new(h, s, l, a).try_into().ok().ok_or(Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn lighten<C>(self, pct: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let pct: q::Percentage<A, B> = pct.into();
        let pct: q::Q<A, B> = pct.into();
        let h: u16 = hsla.h();
        let s: q::Q<A, B> = hsla.s();
        let l: q::Q<A, B> = hsla.l();
        let l_diff: q::Q<A, B> = (l + pct)?;
        let l: q::Q<A, B> = if l_diff > n(1_0)? {
            n(1_0)?
        } else {
            l_diff
        };
        let a: q::Q<A, B> = hsla.a();
        let ret: Hsla<A, B> = Hsla::new(h, s, l, a);
        let ret: Self = ret.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn darken<C>(self, pct: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>> {
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let pct: q::Percentage<A, B> = pct.into();
        let pct: q::Q<A, B> = pct.into();
        let h: u16 = hsla.h();
        let s: q::Q<A, B> = hsla.s();
        let l: q::Q<A, B> = hsla.l();
        let l_diff: q::Q<A, B> = (l - pct)?;
        let l: q::Q<A, B> = if l_diff < n(0_0)? {
            n(0_0)?
        } else {
            l_diff
        };
        let a: q::Q<A, B> = hsla.a();
        let ret: Hsla<A, B> = Hsla::new(h, s, l, a);
        let ret: Self = ret.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn interpolate<C, D>(self, rhs: C, pct: D) -> Result<Self>
    where
        C: Into<Self>,
        D: Into<q::Percentage<A, B>> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let lhs: Hsla<A, B> = self.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let rhs: Self = rhs.into();
        let rhs: Hsla<A, B> = rhs.try_into().ok().ok_or(Error::UnsupportedConversion)?;
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
        let ret: Hsla<A, B> = Hsla::new(h, s, l, a);
        let ret: Self = ret.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        Ok(ret)
    }
}

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIs)]
#[derive(::thiserror::Error)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub enum Error {
    #[error("{0}")]
    ArrayError(#[from] array::Error),
    #[error("{0}")]
    OpsError(#[from] ops::Error),
    #[error("{0}")]
    QError(#[from] q::Error),
    #[error("Illegal hex.")]
    IllegalHex,
    #[error("Alpha overflow.")]
    AlphaOverflow,
    #[error("Alpha underflow.")]
    AlphaUnderflow,
    #[error("")]
    UnsupportedConversion,
    #[error("When casting to another color, the alpha must match exactly.")]
    AlphaTruncation
}

#[cfg_attr(feature = "std", derive(::serde::Serialize))]
#[cfg_attr(feature = "std", derive(::serde::Deserialize))]
#[derive(Clone)]
#[derive(Copy)]
pub struct Color<const A: u8, B = usize, C = HexMode>
where
    B: ops::Int,
    C: Mode,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    mode: C,
    m_0: ::core::marker::PhantomData<B>,
}