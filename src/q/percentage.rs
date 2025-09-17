use super::*;

mode!(
    /// Represents a relative change.
    /// 
    /// Mathematically, a percentage is a normalized value relative to 100:
    /// 
    /// ```text
    /// Percentage = (Factor - 1.0) * 100
    /// ```
    /// 
    /// For example:
    /// 
    /// - `Percentage<2>` with value `50` represents `+0.50%`
    /// - `Percentage<2>` with value `-10000` represents `-100.00%`
    /// - `Percentage<2>` with value `0` represents no change (`0.00%`)
    Percentage
);

impl<const A: u8, B, C> Percentage<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {
    #[inline]
    pub fn is_gain(self) -> bool {
        self > as_100()
    }

    #[inline]
    pub fn is_loss(self) -> bool {
        self < (B::AS_100.into() - (B::AS_100.into() * B::AS_2.into()))
    }

    #[inline]
    pub fn is_neutral(self) -> bool {
        self == as_0()
    }

    #[inline]
    pub fn of<D>(self, n: D) -> Result<Unit<A, B, C>> 
    where
        D: Into<Unit<A, B, C>> {
        let percentage: Unit<_, _, _> = self.into();
        if B::SIGNED {
            let min: Unit<_, _, _> = (as_100() * as_2()).unwrap();
            let min: Unit<_, _, _> = (as_100() - min).unwrap();
            let percentage: Unit<_, _, _> = percentage.max(min);
            let n: Unit<_, _, _> = n.into();
            let n: Unit<_, _, _> = (n / as_100())?;
            let n: Unit<_, _, _> = (n * percentage)?;
            return Ok(n)
        }
        let n: Unit<_, _, _> = n.into();
        let n: Unit<_, _, _> = (n / as_100())?;
        let n: Unit<_, _, _> = (n * percentage)?;
        Ok(n)
    }
}

impl<const A: u8, B, C> From<Unit<A, B, C>> for Percentage<A, B, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Unit<A, B, C>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B, C> TryFrom<Factor<A, B, C>> for Percentage<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {
    type Error = Error;
    
    fn try_from(value: Factor<A, B, C>) -> ::core::result::Result<Self, Self::Error> {
        let n: Unit<_, _, _> = value.into();
        if (n < as_1() && !B::SIGNED) || n < as_0() {
            return Err(Error::Underflow)
        }
        if n == as_1() {
            return Ok(as_0())
        }
        if n > as_1() {
            let n: Unit<_, _, _> = (n - as_1())?;
            let n: Unit<_, _, _> = (n * as_100())?;
            let ret: Self = n.into();
            return Ok(ret)
        }
        let n: Unit<_, _, _> = (n - as_1())?;
        let n: Unit<_, _, _> = (n * as_100())?;
        let ret: Self = n.into();
        Ok(ret)
    }
}