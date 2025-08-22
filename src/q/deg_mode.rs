use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Deg<const A: u8, B> = Q<A, B, DegMode>;
            $(
                pub type [< Deg $n >]<T> = Q<$n, T, DegMode>;
            )*
        );
    };
}

ty!(
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct DegMode;

impl<const A: u8, B, C> TryFrom<Q<A, B, RadMode, C>> for Q<A, B, DegMode, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Q<A, B, RadMode, C>) -> ::core::result::Result<Self, Self::Error> {
        value.to_deg()
    }
}

impl<const A: u8, B, C> Q<A, B, DegMode, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        self.to_rad()?.tan()
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        self.to_rad()?.sin()
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        self.to_rad()?.cos()
    }

    #[inline]
    pub fn to_rad(self) -> Result<Q<A, B, RadMode, C>> {
        let ret: B = self.n;
        let ret: B = C::to_rad(ret)?;
        let ret: Q<A, B, RadMode, C> = ret.into();
        Ok(ret)
    }
}