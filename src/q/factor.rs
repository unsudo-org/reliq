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

impl<const A: u8, B, C> From<B> for Factor<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Unsigned,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: B) -> Self {
        Self {
            n: value,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
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
        let n: Unit<_, _, _> = value.into();
        if n == as_0() {
            return as_0()
        }
        if n > as_0() {
            let n: Unit<_, _, _> = (n / as_100()).unwrap();
            let n: Unit<_, _, _> = (n + as_1()).unwrap();
            let ret: Self = n.into();
            return ret
        }
        let min: Unit<_, _, _> = (as_0() - as_100()).unwrap();
        let n: Unit<_, _, _> = n.max(min);
        let n: Unit<_, _, _> = (n / as_100()).unwrap();
        let n: Unit<_, _, _> = (n + as_1()).unwrap();
        let ret: Self = n.into();
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