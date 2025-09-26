use super::*;

mode!(
    /// ```text
    /// Unit<A, B, C> := n ∈ ℚₐ ≅ ∀ T ∈ Typed<A, B, C>, T.n = n
    /// ```
    /// 
    /// The `Unit` erases semantic meaning, but preserves numeric structure and precision.
    Unit
);

impl<const A: u8, B> Unit<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {

}

impl<const A: u8, B> From<Deg<A, B>> for Unit<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Deg<A, B>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B> From<Rad<A, B>> for Unit<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Rad<A, B>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B> From<Percentage<A, B>> for Unit<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Percentage<A, B>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B> From<Factor<A, B>> for Unit<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Factor<A, B>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B> From<Ratio<A, B>> for Unit<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Ratio<A, B>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B> From<Delta<A, B>> for Unit<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Delta<A, B>) -> Self {
        value.n.into()
    }
}