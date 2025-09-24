use super::*;

pub trait RandDriver {
    fn next<T>(&mut self, range: ::core::ops::Range<T>) -> T
    where
        T: ops::Int;
}

mode!(
    Chance
);

impl<const A: u8, B, C> Chance<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[cfg(not(feature = "std"))]
    pub fn roll<D>(self, driver: &mut D)
    where
        D: RandDriver {
        driver.next(0..2);
    }

    #[cfg(feature = "std")]
    pub fn roll(self) -> Unit<A, B, C> {
        use ops::ToPrim as _;
        let n: B = self.n;
        let n: u128 = unsafe {
            n.to_u128().unwrap_unchecked()
        };
        let n: u128 = ::fastrand::u128(..);
        
    }
}

impl<const A: u8, B, C> From<Unit<A, B, C>> for Chance<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Unit<A, B, C>) -> Self {
        Self {
            n: value.n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Percentage<A, B, C>> for Chance<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Percentage<A, B, C>) -> Self {
        Self {
            n: value.n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}