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
    Self: Sized,
    Self: Clone,
    Self: Copy,
    Self: From<Hsla<A, B>>,
    Self: Into<Hsla<A, B>>,
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn complement(self) -> Result<Self> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };      
        let hsla: Hsla<A, B> = self.into();
        let h: u16 = hsla.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h: q::Q<A, B> = (h + n(180_0)?)?;
        let h: q::Q<A, B> = (h % n(360_0)?)?;
        let h: u16 = h.to_u16()?;
        let s: q::Q<A, B> = hsla.s();
        let l: q::Q<A, B> = hsla.l();
        let a: q::Q<A, B> = hsla.a();
        let ret: Hsla<A, B> = Color {
            mode: HslaMode {
                h,
                s,
                l,
                a
            },
            m_0: ::core::marker::PhantomData
        };
        let ret: Self = ret.into();
        Ok(ret)
    }
    
    #[inline]
    fn triadic(self) -> Result<[Self; 3]> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.into();
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
        let ret_0: Hsla<A, B> = Color {
            mode: HslaMode {
                h: h_0,
                s,
                l,
                a
            },
            m_0: ::core::marker::PhantomData
        };
        let ret_0: Self = ret_0.into();
        let ret_1: Hsla<A, B> = Color {
            mode: HslaMode {
                h: h_1,
                s,
                l,
                a
            },
            m_0: ::core::marker::PhantomData
        };
        let ret_1: Self = ret_1.into();
        let ret_2: Hsla<A, B> = Color {
            mode: HslaMode {
                h: h_2,
                s,
                l,
                a
            },
            m_0: ::core::marker::PhantomData
        };
        let ret_2: Self = ret_2.into();
        let ret: [Self; 3] = [
            ret_0,
            ret_1,
            ret_2
        ];
        Ok(ret)
    }

    fn tetradic(self) -> Result<[Self; 4]> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.into();
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
            let ret: Hsla<A, B> = Color {
                mode: HslaMode {
                    h,
                    s,
                    l,
                    a
                },
                m_0: ::core::marker::PhantomData
            };
            let ret: Self = ret.into();
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

    #[inline]
    fn analogous(self) -> Result<[Self; 12]> {
        use ops::ToPrim as _;
        let n: _ = |x: u32| -> Result<q::Q<A, B>> {
            Ok(q::r#as::<1, _, u32, _, _, u32>(x)?)
        };
        let hsla: Hsla<A, B> = self.into();
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
            let ret: Hsla<A, B> = Color {
                mode: HslaMode {
                    h,
                    s,
                    l,
                    a
                },
                m_0: ::core::marker::PhantomData
            };
            let ret: Self = ret.into();
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
        let hsla: Hsla<A, B> = self.into();
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
            let color: Hsla<A, B> = Color {
                mode: HslaMode {
                    h,
                    s,
                    l,
                    a
                },
                m_0: ::core::marker::PhantomData
            };
            let color: Self = color.into();
            ret.push(color).unwrap();
        }
        Ok(ret)
    }

    fn saturate<C>(self, percentage: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>>;

    fn desaturate<C>(self, percentage: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>>;

    fn lighten<C>(self, percentage: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>>;

    fn darken<C>(self, percentage: C) -> Result<Self>
    where
        C: Into<q::Percentage<A, B>>;

    fn interpolate<C, D>(self, rhs: C, percentage: D) -> Result<Self>
    where
        C: Into<Self>,
        D: Into<q::Percentage<A, B>>;
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