use super::*;

pub type Rgba<const A: q::Precision = 2, B = usize, C = q::DefaultEngine, D = DefaultEngine> = Color<A, B, RgbaMode<A, B, C>, C, D>;

pub struct RgbaMode<const A: q::Precision, B = usize, C = q::DefaultEngine> 
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

impl<const A: q::Precision, B, C, D> From<Rgb<A, B, C, D>> for Rgba<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: Rgb<A, B, C, D>) -> Self {
        let rgb: Rgb<_, _, _, _> = value;
        let r: u8 = *rgb.r();
        let g: u8 = *rgb.g();
        let b: u8 = *rgb.b();
        let a: q::Unit<_, _, _> = B::AS_1.into();
        Self {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        }
    }
}

impl<const A: q::Precision, B, C, D, E> From<(u8, u8, u8, E)> for Rgba<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    E: Into<q::Unit<A, B, C>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: (u8, u8, u8, E)) -> Self {
        let (
            r,
            g,
            b,
            a
        ) = value;
        let a: q::Unit<_, _, _> = a.into();
        Self {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
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
        let (
            r, 
            g, 
            b
        ) = value;
        let a: q::Unit<_, _, _> = B::AS_1.into();
        Self {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        }
    }
}