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

impl<const A: u8, B, C> Factor<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {

    pub fn is_increase(&self) -> bool {
        self.n > B::AS_1
    }

    pub fn is_decrease(&self) -> bool {
        self.n < B::AS_1
    }

    pub fn is_neutral(&self) -> bool {
        self.n == B::AS_1
    }
}

impl<const A: u8, B, C> From<Unit<A, B, C>> for Factor<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Unsigned,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Unit<A, B, C>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B, C> From<Percentage<A, B, C>> for Factor<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {
    #[inline]
    fn from(value: Percentage<A, B, C>) -> Self {
        let n: Unit<A, B, C> = value.into();
        if n == as_0::<A, B, UnitMode, C>() {
            return as_0::<A, B, FactorMode, C>()
        }
        if n > as_0::<A, B, UnitMode, C>() {
            let n: Unit<A, B, C> = ((n / as_100::<A, B, UnitMode, C>()) + as_1::<A, B, UnitMode, C>()).into_result().unwrap();
            let ret: Self = n.into_int().into();
            return ret
        }
        let min: Unit<A, B, C> = (as_0::<A, B, UnitMode, C>() - as_100::<A, B, UnitMode, C>()).into_result().unwrap();
        let n: Unit<A, B, C> = n.max(min);
        let n: Unit<A, B, C> = ((n / as_100::<A, B, UnitMode, C>()) + as_1::<A, B, UnitMode, C>()).into_result().unwrap();
        let ret: Self = n.into_int().into();
        ret
    }
}

#[test]
fn test_percentage_to_factor() {
    let n: Percentage2<usize> = 25_00.into();
    let n: Factor2 = n.into();
    let ok: Factor2 = 1_25.into();
    assert_eq!(n, ok);
}