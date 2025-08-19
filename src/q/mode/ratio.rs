use super::*;

#[repr(transparent)]
pub struct RatioMode;

impl Mode for RatioMode {}

impl<const A: u8, B, C> From<B> for Q<A, B, RatioMode, C>
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

impl<const A: u8, const B: u8, C, D> TryFrom<Q<A, C, DefaultMode, D>> for Q<B, C, RatioMode, D>
where
    C: num::Int,
    D: Engine,
    (): Precision<A>,
    (): Precision<B>,
    (): N<C> {
    type Error = Error;

    #[inline]
    fn try_from(q: Q<A, C, DefaultMode, D>) -> ::core::result::Result<Self, Self::Error> {
        let n: C = q.n;
        let n: C = D::cast::<A, B, _>(n)?;
        let n: Self = Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        };
        Ok(n)
    }
}


impl<const A: u8, const B: u8, C, D> TryFrom<Q<A, C, PercentageMode, D>> for Q<B, C, RatioMode, D>
where
    C: num::Int,
    D: Engine,
    (): Precision<A>,
    (): Precision<B>,
    (): N<C> {
    type Error = Error;

    #[inline]
    fn try_from(q: Q<A, C, PercentageMode, D>) -> ::core::result::Result<Self, Self::Error> {
        let n: C = q.n;
        let n: C = D::cast::<A, B, _>(n)?;
        let n: C = D::div::<B, _>(n, n100())?;
        let n: Self = Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        };
        Ok(n)   
    }
}

impl<const A: u8, B, C> ::core::fmt::Debug for Q<A, B, RatioMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let x = self.n;
        let y = C::sub(B::AS_1, self.n).map_err(|_| ::core::fmt::Error)?;
        write!(f, "[{}]({}:{})", A, x, y)
    }
}