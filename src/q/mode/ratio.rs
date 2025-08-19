use super::*;

#[repr(transparent)]
pub struct RatioMode;

impl Mode for RatioMode {}

impl<const A: u8, B, C> TryFrom<Q<A, B, DefaultMode, C>> for Q<A, B, RatioMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Error = Error;

    fn try_from(q: Q<A, B, DefaultMode, C>) -> ::core::result::Result<Self, Self::Error> {
        let n: B = q.n;
        if n < B::AS_0 {
            return Err()
        }
        if n > n100() {
            return Err()
        }
        Ok(Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        })
    }
}

impl<const A: u8, B, C> TryFrom<Q<A, B, PercentageMode, C>> for Q<A, B, RatioMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Error = Error;

    fn try_from(q: Q<A, B, PercentageMode, C>) -> ::core::result::Result<Self, Self::Error> {
        let n: B = q.n;
        let n: B = unsafe {
            // y is never `0`
            C::div(n, n100()).unwrap_unchecked()
        };
        Ok(Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        })
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