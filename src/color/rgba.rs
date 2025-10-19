use super::*;

pub struct RgbaMode<const A: u8 = 1, B = usize> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    r: u8,
    g: u8,
    b: u8,
    a: q::Q<A, B>
}

impl<const A: u8, B> Mode for RgbaMode<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {}


pub type Rgba<const A: u8 = 1, B = usize> = Color<A, B, RgbaMode<A, B>>;

impl<const A: u8, B> Rgba<A, B>
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
    pub fn a(&self) -> q::Q<A, B> {
        self.mode.a
    }
}

impl<const A: u8, B, C> From<(u8, u8, u8, C)> for Rgba<A, B>
where
    B: ops::Int,
    C: Into<q::Q<A, B>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    
    /// # Warning
    /// If alpha is less than `0` or greater than `1`,
    /// it will be clamped.
    /// 
    /// # Error
    /// `AlphaTruncation`
    #[inline]
    fn from(value: (u8, u8, u8, C)) -> Self {
        let (
            r,
            g,
            b,
            a
        ) = value;
        let min: q::Q<A, B> = q::as_0();
        let max: q::Q<A, B> = q::as_1();
        let a: q::Q<A, B> = a.into();
        let a: q::Q<A, B> = a.clamp(min, max);
        Self {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> From<(u8, u8, u8)> for Rgba<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: (u8, u8, u8)) -> Self {
        let (
            r, 
            g, 
            b
        ) = value;
        let a: q::Q<A, B> = B::AS_1.into();
        Self {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> From<Hex<A, B>> for Rgba<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn from(value: Hex<A, B>) -> Self {
        
    }
}

impl<const A: u8, B> From<Rgb<A, B>> for Rgba<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    #[inline]
    fn from(value: Rgb<A, B>) -> Self {
        let rgb: Rgb<A, B> = value;
        let r: u8 = rgb.r();
        let g: u8 = rgb.g();
        let b: u8 = rgb.b();
        let a: q::Q<A, B> = B::AS_1.into();
        Self {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> CommonExt<A, B> for Rgba<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>
    {}