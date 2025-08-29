use super::*;
pub use super::Result;
pub use super::Error;

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

type QTuple<const A: u8, B, C> = (
    Q<A, B, C>,
    Q<A, B, C>,
    Q<A, B, C>
);
type Q<const A: u8, B, C> = q::Q<A, B, q::DefaultMode, C>;

#[derive(Clone)]
#[derive(Copy)]
pub struct Point3D<const A: u8, B, C = q::DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub x: Q<A, B, C>,
    pub y: Q<A, B, C>,
    pub z: Q<A, B, C>
}

impl<const A: u8, B, C> Point3D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    
}

impl<const A: u8, B, C> From<QTuple<A, B, C>> for Point3D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: QTuple<A, B, C>) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2
        }
    }
}

impl<const A: u8, B, C> Point<A, B, C> for Point3D<A, B, C> 
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn distance_between(self, rhs: Self) -> Result<Q<A, B, C>>
    where
        B: ops::Int,
        B: ops::Prim,
        B: ops::Signed,
        C: q::Engine,
        (): q::SupportedPrecision<A>,
        (): q::SupportedInt<B>,
        (): q::Supported<A, B> {
        let dx: Q<A, B, C> = (self.x - rhs.x)?;
        let dx: Q<A, B, C> = (dx * dx)?;
        let dy: Q<A, B, C> = (self.y - rhs.y)?;
        let dy: Q<A, B, C> = (dy * dy)?;
        let dz: Q<A, B, C> = (self.z - rhs.z)?;
        let dz: Q<A, B, C> = (dz * dz)?;
        let ret: Q<A, B, C> = ((dx + dy)? + dz)?;
        let ret: Q<A, B, C> = ret.sqrt()?;
        Ok(ret)
    }
}