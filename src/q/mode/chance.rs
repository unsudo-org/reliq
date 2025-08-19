use super::*;

// Represents the a chance of something occuring, unlike percentage
// a chance is clamped to 0..=1, and exposes samantically
// meaning interactions. This also means a chance can store more
// accurate values becauses of this limited range, but when
// converting from percentage it will loose info on ie 250% -> 1
// so conversion back will result in data loss, and thus is
// discouraged
// Any value outside this range will fail
// and the value must be clamped prior to conversion.
#[repr(transparent)]
pub struct ChanceMode;

impl Mode for ChanceMode {}

impl<const A: u8, B, C> Q<A, B, ChanceMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    pub fn parse(n: B) -> Result<Self> {
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

impl<const A: u8, B, C> From<B> for Q<A, B, ChanceMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    fn from(n: B) -> Self {
        let min: B = unsafe {
            C::cast(n0()).unwrap_unchecked()
        };
        let max: B = unsafe {
            C::cast(n100()).unwrap_unchecked()
        };
        let n: B = n.clamp(min, max);
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Q<A, B, PercentageMode, C>> for Q<A, B, ChanceMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    fn from(q: Q<A, B, PercentageMode, C>) -> Self {
        let min: B = unsafe {
            C::cast(n0()).unwrap_unchecked()
        };
        let max: B = unsafe {
            C::cast(n100()).unwrap_unchecked()
        };
        let n: B = q.n;
        let n: B = n.clamp(min, max);
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> ::core::ops::Add for Q<A, B, ChanceMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let p0: B = self.n;
        let p1: B = rhs.n;
        let n: B = B::AS_1 - (B::AS_1 - p0) * (B::AS_1 - p1);
        Ok(Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        })
    }
}