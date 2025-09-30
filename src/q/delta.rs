use super::*;

mode!(
    /// ```text
    /// Δx = x₂ - x₁
    /// ```
    /// 
    /// A signed value that represents change.
    /// A value of `3.0` or `-2.0` models change in a property.
    /// A value of `0.0` represents no change.
    /// 
    /// Mathematically, it's analogous to the Greek letter Δ (delta), which denotes difference:
    /// 
    /// For example:
    /// 
    /// - `Delta<2> or Delta2` with value `300` -> `+3.00`.
    /// - `Delta<2> or Delta2` with value `-30` -> `-0.30`.
    /// - `Delta<2> or Delta2` with value `0` -> `0.00` (no change).
    /// 
    /// # Semantics
    /// Use `Delta` to represent signed adjustments or differential values, such as:
    /// 
    /// - `Δv` - Change in velocity.
    /// - `Δp` - Price delta.
    /// - `Δθ` - Angle correction.
    /// - `Δerror` - Control system error correction.
    /// 
    /// # Example
    /// 
    /// - `delta_from_raw_signed_int(5_00)`     -> `+5.00`
    /// 
    /// # Precision
    /// 
    /// - `Delta2` -> 2 decimal places (`Delta<2>`)
    /// - `Delta4` -> 4 decimal places (`Delta<4>`)
    /// 
    /// # See Also
    /// 
    /// Greek letter Δ (*delta*)
    /// 
    /// ```text
    /// Δx = x₂ - x₁
    /// ```
    Delta
);

impl<const A: u8, B> Delta<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {

    /// Converts a `Delta` into a `Percentage`, relative to a given base value.
    /// 
    /// # Formula
    /// 
    /// ```text
    /// Percentage = (Delta / Base) * 100
    /// ```
    /// 
    /// # Safety
    /// 
    /// - The base can not be 0.
    pub fn into_percentage<D>(self, base: D) -> Result<Percentage<A, B>> 
    where
        B: ops::Signed,
        D: Into<Unit<A, B>> {
        let base: Unit<A, B> = base.into();
        if base.n == B::AS_0 {
            return Err(Error::DivisionByZero)
        }
        let ret: Unit<A, B> = self.into();
        let ret: Unit<A, B> = (ret * as_100::<_, _, UnitMode>())?;
        let ret: Unit<A, B> = (ret / base)?;
        let ret: Percentage<A, B> = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B> From<Unit<A, B>> for Delta<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Unit<A, B>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B> TryFrom<Factor<A, B>> for Delta<A, B> 
where
    B: ops::Int,
    B: ops::Signed,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Factor<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let ret: Self = value.n.checked_sub(B::AS_1).ok_or(Error::Underflow)?.into();
        Ok(ret)
    }
}