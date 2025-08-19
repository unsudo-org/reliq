use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        pub type Rad<const A: u8, B> = Q<A, B, RadMode>;
        $(
            ::paste::paste!(
                pub type [< Rad $n >]<T> = Q<$n, T, RadMode>;
            );
        )*
    };
}

ty!(
    1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

#[repr(transparent)]
pub struct RadMode;

impl<const A: u8, B, C> Q<A, B, RadMode, C> 
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    pub fn tan(self) -> Result<Q<A, B, RatioMode, C>> {
        Ok(Q::new(C::tan(self.n)?))
    }

    #[inline]
    pub fn sin(self) -> Result<Q<A, B, RatioMode, C>> {
        Ok(Q::new(C::sin(self.n)?))
    }

    #[inline]
    pub fn cos(self) -> Result<Q<A, B, RatioMode, C>> {
        Ok(Q::new(C::cos(self.n)?))
    }
}

impl<const A: u8, B, C> TryFrom<Q<A, B, DegMode, C>> for Q<A, B, RadMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Error = Error;

    fn try_from(value: Q<A, B, DegMode, C>) -> ::core::result::Result<Self, Self::Error> {
        Ok(Q::new(C::to_rad(value.n)?))
    }
}