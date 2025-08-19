use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        pub type Deg<const A: u8, B> = Q<A, B, DegMode>;
        $(
            ::paste::paste!(
                pub type [< Deg $n >]<T> = Q<$n, T, DegMode>;
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
pub struct DegMode;

impl Mode for DegMode {}

impl<const A: u8, B, C> Q<A, B, DegMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    pub fn tan(self) -> Result<Q<A, B, RatioMode, C>> {
        let q: Q<A, B, RadMode, C> = self.try_into()?;
        let n: B = q.n;
        let n: B = C::tan(n)?;
        let n: Q<A, B, RatioMode, C> = n.into();
        Ok(n)
    }

    #[inline]
    pub fn sin(self) -> Result<Q<A, B, RatioMode, C>> {
        let q: Q<A, B, RadMode, C> = self.try_into()?;
        let n: B = q.n;
        let n: B = C::sin(n)?;
        let n: Q<A, B, RatioMode, C> = n.into();
        Ok(n)
    }

    #[inline]
    pub fn cos(self) -> Result<Q<A, B, RatioMode, C>> {
        let q: Q<A, B, RadMode, C> = self.try_into()?;
        let n: B = q.n;
        let n: B = C::cos(n)?;
        let n: Q<A, B, RatioMode, C> = n.into();
        Ok(n)
    }
}

impl<const A: u8, B, C> From<B> for Q<A, B, DegMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    fn from(n: B) -> Self {
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    } 
}

impl<const A: u8, B, C> From<Q<A, B, DefaultMode, C>> for Q<A, B, DegMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    fn from(q: Q<A, B, DefaultMode, C>) -> Self {
        q.n.into()
    }
}

impl<const A: u8, B, C> TryFrom<Q<A, B, RadMode, C>> for Q<A, B, DegMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Error = Error;

    #[inline]
    fn try_from(q: Q<A, B, RadMode, C>) -> ::core::result::Result<Self, Self::Error> {
        let n: B = q.n;
        let n: B = C::to_deg(n)?;
        let n: Self = n.into();
        Ok(n)
    }
}