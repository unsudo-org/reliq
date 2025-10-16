use super::*;

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

pub type Hsla<const A: u8 = 1, B = usize> = Color<A, B, HslaMode<A, B>>;

impl<const A: u8, B> Hsla<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>  {
    #[inline]
    pub fn new<C, D, E, F>(
        h: C,
        s: D,
        l: E,
        a: F
    ) -> Self 
    where
        C: Into<u16>,
        D: Into<q::Q<A, B>>,
        E: Into<q::Q<A, B>>,
        F: Into<q::Q<A, B>> {
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

    #[inline]
    pub fn h(&self) -> u16 {
        self.mode.h
    }

    #[inline]
    pub fn s(&self) -> q::Q<A, B> {
        self.mode.s
    }

    #[inline]
    pub fn l(&self) -> q::Q<A, B> {
        self.mode.l
    }

    #[inline]
    pub fn a(&self) -> q::Q<A, B> {
        self.mode.a
    }
}

impl<const A: u8, B, C, D, E, F> From<(C, D, E, F)> for Hsla<A, B>
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

impl<const A: u8, B> TryFrom<Hex<A, B>> for Hsla<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Error = Error;

    fn try_from(value: Hex<A, B>) -> ::core::result::Result<Self, Self::Error> {
        
    }
}

impl<const A: u8, B> TryFrom<Hsl<A, B>> for Hsla<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Error = Error;

    fn try_from(value: Hsl<A, B>) -> ::core::result::Result<Self, Self::Error> {
        
    }
}

impl<const A: u8, B> CommonExt<A, B> for Hsla<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> 
    {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn complement() {
        let x: Hsla<2, u128> = (200u16, 20_00, 40_00, 20_00).into();
        let x_complement: Hsla<2, u128> = x.complement().unwrap();
        let x_complement_h: u16 = x_complement.h();
        assert_eq!(x_complement_h, 20);
    }
}