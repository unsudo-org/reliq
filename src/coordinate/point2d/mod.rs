use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            $(
                pub type [< Point2D $n >]<A> = Point2D<$n, A>;
            )*
        );
    };
}

ty!(
    1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

#[derive(Clone)]
#[derive(Copy)]
pub struct Point2D<const A: u8, B, C = q::DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub x: q::Q<A, B, q::DefaultMode, C>,
    pub y: q::Q<A, B, q::DefaultMode, C>
}

impl<const A: u8, B, C> From<(B, B)> for Point2D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: (B, B)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into()
        }
    }
}

impl<const A: u8, B, C> From<(q::Q<A, B, q::DefaultMode, C>, q::Q<A, B, q::DefaultMode, C>)> for Point2D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: (q::Q<A, B, q::DefaultMode, C>, q::Q<A, B, q::DefaultMode, C>)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}