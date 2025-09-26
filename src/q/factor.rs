use super::*;

mode!(
    /// Represents a multiplicative scaling factor.
    ///
    /// Mathematically:
    /// ```
    /// Factor = 1.0 + (Percentage / 100)
    /// ```
    ///
    /// A `Factor` is commonly used for scaling, compounding, and normalizing values.
    /// Unlike `Percentage`, it is **multiplicative**, not additive.
    ///
    /// # Examples
    /// - `1.00` → no change (`0%`)
    /// - `1.50` → +50% increase
    /// - `0.75` → -25% decrease
    ///
    /// # Usage
    /// Use `Factor` when you need:
    /// - Growth or shrinkage ratios
    /// - Price multipliers (e.g. taxes, discounts)
    /// - Compounding interest multipliers
    Factor
);

impl<const A: u8, B> Factor<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<0, B> {

    #[inline]
    pub fn is_increase(&self) -> bool {
        let as_1: Self = as_1();
        self > &as_1
    }

    #[inline]
    pub fn is_decrease(&self) -> bool {
        let as_1: Self = as_1();
        self < &as_1
    }

    #[inline]
    pub fn is_neutral(&self) -> bool {
        let as_1: Self = as_1();
        self == &as_1
    }
}

impl<const A: u8, B> From<Unit<A, B>> for Factor<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Unsigned,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn from(value: Unit<A, B>) -> Self {
        let n: Unit<A, B> = value;
        let n: Self = n.into();
        n
    }
}

impl<const A: u8, B> From<Percentage<A, B>> for Factor<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<0, B> {
    #[inline]
    fn from(value: Percentage<A, B>) -> Self {
        let n: Unit<A, B> = value.into();
        let one: Unit<A, B> = as_1();
        let one_hundred: Unit<A, B> = as_100();
        let n: Unit<A, B> = if B::SIGNED {
            let min: Unit<A, B> = unsafe {
                (as_0() - one_hundred).unwrap_unchecked()
            };
            n.max(min)
        } else {
            n
        };
        let n: Unit<A, B> = unsafe {
            (n / one_hundred).unwrap_unchecked()
        };
        let n: Unit<A, B> = unsafe {
            (n + one).unwrap_unchecked()
        };
        let n: Self = Self::from_raw(n.n);
        n
    }
}

#[cfg(test)]
#[allow(clippy::inconsistent_digit_grouping)]
mod test {
    use super::*;

    #[::rstest::rstest]
    #[case(25_00, 1_25)]
    #[case(0_00, 1_00)]
    #[case(-25_00, 0_75)]
    #[case(-100_00, 0_00)]
    #[case(-200_00, 0_00)]
    #[case(450_00, 3_50)]
    fn percentage_to_factor<A, B>(#[case] percentage: A, #[case] ok: B) 
    where
        A: Into<Percentage2<isize>>,
        B: Into<Factor2<isize>> {
        let n: Percentage2<isize> = percentage.into();
        let n: Factor2<isize> = n.into();
        let ok: Factor2<isize> = ok.into();
        assert_eq!(n, ok);
    }
}