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

impl<const A: u8, B, C> From<B> for Q<A, B, DegMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    fn from(n: B) -> Self {
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    } 
}

impl<const A: u8, const B: u8, C, D> TryFrom<Q<B, C, DefaultMode, D>> for Q<A, C, DegMode, D>
where
    C: num::Int,
    D: Engine,
    (): Precision<A>,
    (): Precision<B>,
    (): N<C> {
    type Error = Error;

    fn try_from(q: Q<B, C, DefaultMode, D>) -> ::core::result::Result<Self, Self::Error> {
        let n: C = q.n;
        let n: C = D::cast::<A, B, _>(n)?;
        let n: Self = n.into();
        Ok(n)
    }
}

impl<const A: u8, const B: u8, C, D> TryFrom<Q<B, C, RadMode, D>> for Q<A, C, DegMode, D>
where
    C: num::Int,
    D: Engine,
    (): Precision<A>,
    (): Precision<B>,
    (): N<C> {
    type Error = Error;
    
    fn try_from(q: Q<B, C, RadMode, D>) -> ::core::result::Result<Self, Self::Error> {
        let n: C = q.n;
        let n: C = D::cast::<A, B, _>(n)?;
        let n: C = D::to_deg::<B, _>(n)?;
        let n: Self = n.into();
        Ok(n)
    }
}