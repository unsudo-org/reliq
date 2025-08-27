use super::*;

::modwire::expose!(
    pub error
);

#[repr(u8)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Color<const A: u8, B> 
where
    B: ops::Int,
    B: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    Hex(u32),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, q::Q<A, B>)
}

impl<const A: u8, B> Color<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    
}

impl<const A: u8, B> From<(u8, u8, u8)> for Color<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: (u8, u8, u8)) -> Self {
        let r: u8 = value.0;
        let g: u8 = value.1;
        let b: u8 = value.2;
        Self::Rgb(r, g, b)
    }
}

impl<const A: u8, B> From<u32> for Color<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: u32) -> Self {
        if value > 0xffffff {
            Self::Hex(0xffffff)
        } else {
            Self::Hex(value)
        }
    }
}

impl<const A: u8, B, C> From<(u8, u8, u8, C)> for Color<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    C: Into<q::Q<A, B>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>,
    (): q::Supported<1, B> {
    fn from(value: (u8, u8, u8, C)) -> Self {
        let r: u8 = value.0;
        let g: u8 = value.1;
        let b: u8 = value.2;
        let a: q::Q<A, B> = value.3.into();
        let a: q::Q<A, B> = a.clamp(q::as_0(), q::as_1());
        Self::Rgba(r, g, b, a)
    }
}

impl<const A: u8, B> ::core::fmt::Display for Color<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Hex(code) => write!(f, "#{:06X}", code),
            Self::Rgb(r, g, b) => write!(f, "rgb({}, {}, {})", r, g, b),
            Self::Rgba(r, g, b, a) => write!(f, "rgba({}, {}, {}, {:.3})", r, g, b, a)
        }
    }
}