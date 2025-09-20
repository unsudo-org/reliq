use super::*;

pub type Rgba<const A: q::Precision = 2, B  = usize, C = q::DefaultEngine> = Color<A, B, RgbaMode<A, B, C>, C>;

pub struct RgbaMode<const A: q::Precision, B, C> 
where
    B: ops::Int,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    r: u8,
    g: u8,
    b: u8,
    a: q::Unit<A, B, C>
}

impl<const A: q::Precision, B, C> Mode for RgbaMode<A, B, C> 
where
    B: ops::Int,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {}

impl<const A: q::Precision, B, C, D> From<Rgb<A, B, C>> for Rgba<A, B, C>
where
    B: ops::Int,
    C: q::Engine,
    D: Into<q::Unit<A, B, C>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: Rgb<A, B, C>) -> Self {
        let r: u8 = value.mode.r;
        let g: u8 = value.mode.g;
        let b: u8 = value.mode.b;
        let a: q::Unit<_, _, _> = B::AS_1.into();
        Self {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: q::Precision, B, C, D> From<(u8, u8, u8, D)> for Rgba<A, B, C>
where
    B: ops::Int,
    C: q::Engine,
    D: Into<q::Unit<A, B, C>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: (u8, u8, u8, D)) -> Self {
        let r: u8 = value.0;
        let g: u8 = value.1;
        let b: u8 = value.2;
        let a: q::Unit<_, _, _> = value.3.into();
        Self {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: q::Precision, B, C> From<(u8, u8, u8)> for Rgba<A, B, C>
where
    B: ops::Int,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: (u8, u8, u8)) -> Self {
        let r: u8 = value.0;
        let g: u8 = value.1;
        let b: u8 = value.2;
        let a: q::Unit<_, _, _> = B::AS_1.into();
        Self {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}