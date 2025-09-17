use super::*;

mode!(
    /// ```text
    /// Unit<A, B, C> := n ∈ ℚₐ ≅ ∀ T ∈ Typed<A, B, C>, T.n = n
    /// ```
    /// 
    /// The `Unit` erases semantic meaning, but preserves numeric structure and precision.
    Unit
);

impl<const A: u8, B, C> Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {

}

impl<const A: u8, B, C> From<Deg<A, B, C>> for Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Deg<A, B, C>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B, C> From<Rad<A, B, C>> for Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Rad<A, B, C>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B, C> From<Percentage<A, B, C>> for Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Percentage<A, B, C>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B, C> From<Factor<A, B, C>> for Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Factor<A, B, C>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B, C> From<Ratio<A, B, C>> for Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Ratio<A, B, C>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B, C> From<Delta<A, B, C>> for Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Delta<A, B, C>) -> Self {
        value.n.into()
    }
}