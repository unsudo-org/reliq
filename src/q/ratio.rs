use super::*;

mode!(
    /// ```text
    /// r = x₂ / x₁
    /// ```
    /// 
    /// Represents a proportional value.
    /// 
    /// A *unitless*, positive value used
    /// 
    Ratio
);

    impl<const A: u8, B, C> Ratio<A, B, C> 
    where
        B: ops::Int,
        C: Engine,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        #[inline]
        pub fn invert(self) -> Result<Self> {
            let ret: B = B::AS_1;
            let ret: Self = C::div(ret, self.n)?.into();
            Ok(ret)
        }
    }

    impl<const A: Precision, B, C> From<Unit<A, B, C>> for Ratio<A, B, C>
    where
        B: ops::Int,
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