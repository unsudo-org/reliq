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

impl<const A: u8, B> Ratio<A, B> 
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn invert(self) -> Result<Self> {
        let ret: B = B::AS_1;
        let ret: Self = div(ret, self.n)?.into();
        Ok(ret)
    }
}

impl<const A: u8, B> From<Unit<A, B>> for Ratio<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Unit<A, B>) -> Self {
        Self::from_raw(value.n)
    }
}