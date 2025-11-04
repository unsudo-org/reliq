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

impl<const A: u8, B> Percentage<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {
    #[inline]
    pub fn is_gain(self) -> bool {
        let as_0: Percentage<A, B> = as_0();
        self > as_0
    }

    #[inline]
    pub fn is_loss(self) -> bool {
        let as_0: Percentage<A, B> = as_0();
        self < as_0
    }

    #[inline]
    pub fn is_neutral(self) -> bool {
        let as_0: Percentage<A, B> = as_0();
        self == as_0
    }

    #[inline]
    pub fn of<C>(self, n: C) -> Result<Unit<A, B>> 
    where
        C: Into<Unit<A, B>> {
        let percentage: Unit<A, B> = self.into();
        let as_100: Unit<A, B> = as_100();
        if B::SIGNED {
            let as_2: Unit<A, B> = as_2();
            let min: Unit<A, B> = (as_100 * as_2)?;
            let min: Unit<A, B> = (as_100 - min)?;
            let percentage: Unit<A, B> = percentage.max(min);
            let n: Unit<A, B> = n.into();
            let n: Unit<A, B> = (n / as_100)?;
            let n: Unit<A, B> = (n * percentage)?;
            return Ok(n)
        }
        let n: Unit<A, B> = n.into();
        let n: Unit<A, B> = (n / as_100)?;
        let n: Unit<A, B> = (n * percentage)?;
        Ok(n)
    }
}

impl<const A: u8, B> From<Unit<A, B>> for Percentage<A, B> 
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Unit<A, B>) -> Self {
        Self {
            n: value.n,
            mode: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B> TryFrom<Factor<A, B>> for Percentage<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {
    type Error = Error;
    
    #[inline]
    fn try_from(value: Factor<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let n: Factor<A, B> = value;
        let n: B = n.n;
        let n: Unit<A, B> = Q {
            n,
            mode: ::core::marker::PhantomData
        };
        let as_0: Unit<A, B> = as_0();
        let as_1: Unit<A, B> = as_1();
        let as_100: Unit<A, B> = as_100();
        if (n < as_1 && !B::SIGNED) || n < as_0 {
            return Err(Error::Underflow)
        }
        if n == as_1 {
            let n: Percentage<A, B> = as_0.into();
            return Ok(n)
        }
        if n > as_1 {
            let n: Unit<A, B> = (n - as_1)?;
            let n: Unit<A, B> = (n * as_100)?;
            let n: Self = n.into();
            return Ok(n)
        }
        let n: Unit<A, B> = (n - as_1)?;
        let n: Unit<A, B> = (n * as_100)?;
        let n: B = n.n;
        let n: Self = Self {
            n,
            mode: ::core::marker::PhantomData
        };
        Ok(n)
    }
}


mod hello_world {
    
}