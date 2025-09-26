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

impl<const A: u8, B> From<Unit<A, B>> for Factor<A, B>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Unsigned,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Unit<A, B>) -> Self {
        let n: Unit<_, _> = value;
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
        let n: Percentage<A, B> = value;
        let n: Unit<A, B> = n.into();
        let as_0: Unit<A, B> = as_0();
        let as_1: Unit<A, B> = as_1();
        let as_100: Unit<A, B> = as_100();
        if n == as_0 {
            let n: B = n.n;
            let n: Self = Self::from_raw(n);
            return n
        }
        if n > as_0 {
            let n: Unit<A, B> = unsafe {
                (n / as_100).unwrap_unchecked()
            };
            
        }
        let min: Unit<A, B> = unsafe {
            
        };


        if n == as_0::<A, B, UnitMode>() {
            return as_0()
        }
        if n > as_0() {
            let n: Unit<A, B> = unsafe {
                (n / as_100).unwrap_unchecked()
            };
            let n: Unit<A, B, C> = unsafe {
                (n + as_1::<A, B, UnitMode, C>()).unwrap_unchecked()
            };
            let ret: Self = n.into_int().into();
            return ret
        }
        let min: Unit<A, B, C> = unsafe {
            (as_0::<A, B, UnitMode, C>() - as_100::<A, B, UnitMode, C>()).unwrap_unchecked()
        };
        let n: Unit<A, B, C> = n.max(min);
        let n: Unit<A, B, C> = unsafe {
            (n / as_100::<A, B, UnitMode, C>()).unwrap_unchecked()
        };
        let n: Unit<A, B, C> = unsafe {
            (n + as_1::<A, B, UnitMode, C>()).unwrap_unchecked()
        };
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