use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct RgbMode {
    r: u8,
    g: u8,
    b: u8
}

impl Mode for RgbMode {}

pub type Rgb<const A: u8, B> = Color<A, B, RgbMode>;

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
}

impl<const A: u8, B> Default for Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn default() -> Self {
        (0, 0, 0).into()
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

pub type LossyRgb<const A: u8, B> = lossy::Lossy<Rgb<A, B>>;

impl<const A: u8, B> From<Rgba<A, B>> for LossyRgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    fn from(value: Rgba<A, B>) -> Self {
        let r: u8 = value.r();
        let g: u8 = value.g();
        let b: u8 = value.b();
        let a: q::Q<A, B> = value.a();
        let as_1: q::Q<A, B> = q::as_1();
        if a != as_1 {
            return Err(Error::AlphaTruncation)
        }
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

impl<const A: u8, B> CommonExt<A, B> for Rgb<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>
    {}