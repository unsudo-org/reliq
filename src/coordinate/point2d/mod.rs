use super::ops;
use super::common;
use super::q;

::modwire::expose!(
    pub e
);

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

type QTuple<const A: Precision, B, C> = (
    Q<A, B, C>,
    Q<A, B, C>
);
type Q<const A: Precision, B, C> = q::Q<A, B, q::DefaultMode, C>;
type Precision = u8;

#[derive(Clone)]
#[derive(Copy)]
pub struct Point2D<const A: Precision, B, C = q::DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub x: Q<A, B, C>,
    pub y: Q<A, B, C>
}

impl<const A: Precision, B, C> From<(B, B)> for Point2D<A, B, C>
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

impl<const A: Precision, B, C> From<QTuple<A, B, C>> for Point2D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: QTuple<A, B, C>) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}

impl<const A: Precision, B, C> common::SignedPoint<A, B, C, Error> for Point2D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn distance_between(self, rhs: Self) -> ::core::result::Result<Q<A, B, C>, Error> {
        let dx: Q<A, B, C> = (self.x - rhs.x)?;
        let dx: Q<A, B, C> = (dx * dx)?;
        let dy: Q<A, B, C> = (self.y - rhs.y)?;
        let dy: Q<A, B, C> = (dy * dy)?;
        let ret: Q<A, B, C> = (dx + dy)?;
        let ret: Q<A, B, C> = ret.sqrt()?;
        Ok(ret)
    }
}