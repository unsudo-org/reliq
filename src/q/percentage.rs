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
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {
    #[inline]
    pub fn is_gain(self) -> bool {
        self > as_0::<A, B, PercentageMode, C>()
    }

    #[inline]
    pub fn is_loss(self) -> bool {
        self < as_0::<A, B, PercentageMode, C>()
    }

    #[inline]
    pub fn is_neutral(self) -> bool {
        self == as_0::<A, B, PercentageMode, C>()
    }

    #[inline]
    pub fn of<D>(self, n: D) -> Result<Unit<A, B, C>> 
    where
        D: Into<Unit<A, B, C>> {
        let percentage: Unit<_, _, _> = self.into();
        if B::SIGNED {
            let min: Fragment<A, B, UnitMode, C> = as_100::<A, B, UnitMode, C>() * as_2::<A, B, UnitMode, C>();
            let min: Fragment<A, B, UnitMode, C> = as_100::<A, B, UnitMode, C>() - min;
            let min: Unit<A, B, C> = min.ok().unwrap();
            let percentage: Unit<A, B, C> = percentage.max(min);
            let n: Unit<_, _, _> = n.into();
            let n: Unit<_, _, _> = (n / as_100::<A, B, UnitMode, C>())?;
            let n: Unit<_, _, _> = (n * percentage)?;
            return Ok(n)
        }
        let n: Unit<_, _, _> = n.into();
        let n: Unit<_, _, _> = (n / as_100::<A, B, UnitMode, C>())?;
        let n: Unit<_, _, _> = (n * percentage)?;
        Ok(n)
    }
}

impl<const A: u8, B, C> From<Unit<A, B, C>> for Percentage<A, B, C> 
where
    B: ops::Int,
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
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {
    type Error = Error;

    fn try_from(value: Factor<A, B, C>) -> ::core::result::Result<Self, Self::Error> {
        let n: Unit<A, B, C> = value.into();
        if (n < as_1::<A, B, UnitMode, C>() && !B::SIGNED) || n < as_0::<A, B, UnitMode, C>() {
            return Err(Error::Underflow)
        }
        if n == as_1::<A, B, UnitMode, C>() {
            return Ok(as_0())
        }
        if n > as_1::<A, B, UnitMode, C>() {
            let n: Unit<A, B, C> = (n - as_1::<A, B, UnitMode, C>())?;
            let n: Unit<A, B, C> = (n * as_100::<A, B, UnitMode, C>())?;
            let ret: Self = n.into();
            return Ok(ret)
        }
        let n: Unit<A, B, C> = (n - as_1::<A, B, UnitMode, C>())?;
        let n: Unit<A, B, C> = (n * as_100::<A, B, UnitMode, C>())?;
        let ret: Self = n.into();
        Ok(ret)
    }
}