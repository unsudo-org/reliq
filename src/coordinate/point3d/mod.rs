use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            $(
                pub type [< Point3D $n >]<A> = Point3D<$n, A>;
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

type Precision = u8;

pub type Result<T> = ::core::result::Result<T, Error>;

pub enum Error {}

#[derive(Clone)]
#[derive(Copy)]
pub struct Point3D<const A: Precision, B, C = q::DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub x: q::Q<A, B, q::DefaultMode, C>,
    pub y: q::Q<A, B, q::DefaultMode, C>,
    pub z: q::Q<A, B, q::DefaultMode, C>
}

impl<const A: Precision, B, C> Point3D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub fn distance_between(self, rhs: Self) -> Result<q::Q<A, B, q::DefaultMode, C>> {
        
    }
}
