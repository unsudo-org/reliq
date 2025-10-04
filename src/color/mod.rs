use super::*;

::modwire::expose!(
    pub hex
    pub hsl
    pub hsla
    pub mode
    pub rgb
    pub rgba
);

pub trait CommonExt<const A: u8, B>
where
    Self: Sized,
    Self: From<Hsla<A, B>>,
    Self: Into<Hsla<A, B>>,
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn complement(self) -> Result<Self> {
        use ops::ToPrim as _;
        let n180: q::Q<A, B> = q::r#as::<1, _, u32, _, _, u32>(1800)?;
        let n360: q::Q<A, B> = q::r#as::<1, _, u32, _, _, u32>(3600)?;       
        let hsl: Hsl<A, B> = self.into();
        let h: u16 = hsl.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h: q::Q<A, B> = (h + n180)?;
        let h: q::Q<A, B> = (h % n360)?;
        let h: u16 = h.to_u16()?;
        let s: q::Q<A, B> = hsl.s();
        let l: q::Q<A, B> = hsl.l();
        let ret: Hsl<A, B> = Color {
            mode: HslMode {
                h,
                s,
                l
            },
            m_0: ::core::marker::PhantomData
        };
        let ret: Self = ret.into();
        Ok(ret)
    }
    
    #[inline]
    fn triadic(self) -> Result<[Self; 3]> {
        use ops::ToPrim as _;
        let n360: q::Q<A, B> = q::r#as::<1, _, u32, _, _, u32>(3600)?;
        let n120: q::Q<A, B> = q::r#as::<1, _, u32, _, _, u32>(1200)?;
        let hsla: Hsla<A, B> = self.into();
        let h: u16 = hsla.h();
        let h: B = h.try_into().ok().ok_or(Error::UnsupportedConversion)?;
        let h: q::Q<A, B> = h.into();
        let h_0: q::Q<A, B> = h;
        let h_0: u16 = h_0.to_u16()?;
        let h_1: q::Q<A, B> = (h + n120)?;
        let h_1: q::Q<A, B> = (h_1 % n360)?;
        let h_1: u16 = h_1.to_u16()?; 
        let h_2: q::Q<A, B> = (h + n120)?;
        let h_2: q::Q<A, B> = (h_2 + n120)?;
        let h_2: q::Q<A, B> = (h_2 % n360)?;
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

    fn tetradic(self) -> Result<Self>;
    fn analogous(self) -> Result<Self>;
    fn monochromatic<const C: usize>(self) -> array::Array<C, Self>;

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