use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            /// Represents a `Signed` value that expresses change.
            /// A value of `3.0` or `-2.0` models change in a property.
            /// A value of `0.0` represents no change.
            /// 
            /// Mathematically, it's analogous to the Greek letter Δ (delta), which denotes difference:
            /// 
            /// ```text
            /// Δx = x₂ - x₁
            /// ```
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
            /// # Alias
            /// - `Delta2` -> `Delta<2>` - A `2` decimal precision `Delta`.
            /// - `Delta4` -> `Delta<4>` - A `4` decimal precision `Delta`.
            pub type Delta<const A: u8, B = isize, C = DefaultEngine> = Q<A, B, DeltaMode, C>;
            
            $(
                /// Represents a `Signed` value that expresses change.
                /// A value of `3.0` or `-2.0` models change in a property.
                /// A value of `0.0` represents no change.
                /// 
                /// Mathematically, it's analogous to the Greek letter Δ (delta), which denotes difference:
                /// 
                /// ```text
                /// Δx = x₂ - x₁
                /// ```
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
                /// # Alias
                /// - `Delta2` -> `Delta<2>` - A `2` decimal precision `Delta`.
                /// - `Delta4` -> `Delta<4>` - A `4` decimal precision `Delta`.
                pub type [< Delta $n >]<A = isize, B = DefaultEngine> = Delta<$n, A, B>;
            )*
        );
    };
}

ty!(
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

#[derive(Clone)]
#[derive(Copy)]
pub struct DeltaMode;

impl<const A: u8, B, C> Delta<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {
    pub fn anyhow(self) -> Unit<A, B, C> {
        self.into()
    }

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
    pub fn to_percentage<D>(self, base: D) -> Result<Percentage<A, B, C>> 
    where
        D: Into<Unit<A, B, C>> {
        let base: Unit<_, _, _> = base.into();
        if base.n == B::AS_0 {
            return Err(Error::DivisionByZero)
        }
        let ret: Unit<_, _, _> = self.anyhow();
        let ret: Unit<_, _, _> = ((ret * as_100())? / base)?;
        let ret: Percentage<_, _, _> = ret.into();
        Ok(ret)
    }

    pub fn to_percentage_as_base_one(self) {
        
    }
}

impl<const A: u8, B, C> TryFrom<Factor<A, B, C>> for Delta<A, B, C> 
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Factor<A, B, C>) -> ::core::result::Result<Self, Self::Error> {
        let ret: Self = value.n.checked_sub(B::AS_1).ok_or(Error::Underflow)?.into();
        Ok(ret)
    }
}

/// Construct a `Delta` directly from a signed integer.
/// 
/// # Note
/// 
/// - No scaling is applied.
/// - Expects the integer to be precision-adjusted.
/// 
/// # Example
/// 
/// ```rs
/// let delta: Delta2 = delta_from_raw_signed_int(1_25); // `1.25`.
/// ```
pub fn delta_from_raw_signed_int<const A: u8, B, C>(n: B) -> Delta<A, B, C> 
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    n.into()
}

/// Construct a `Delta` from a signed integer.
/// 
/// For example, `148` will become a `Delta2` representing `148.00`.
/// 
/// # Example
/// 
/// ```rs
/// let delta: Delta2 = delta_from_signed_int(148); // `148.00`.
/// ```
pub fn delta_from_signed_int<const A: u8, B, C>(n: B) -> Result<Delta<A, B, C>>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    let ret: B = n.checked_mul(scale::<A, B>()).ok_or(Error::Overflow)?;
    let ret: Delta<_, _, _> = ret.into();
    Ok(ret)
}

/// Construct a `Delta` from a floating-point value.
/// 
/// The floating is scaled to the appropriate integer representation based on precision.
pub fn delta_from_float<const A: u8, B, C, D>(n: C) -> Result<Delta<A, B, D>>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: ops::Float,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    
    let ret: B = 
}

#[test]
fn test_delta_from_float() {
    let n: f32 = 3.45;
    let _: Delta2 = delta_from_float(n).unwrap();
}