use super::*;

impl<const A: u8, B, C> Eq for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {}

impl<const A: u8, B, C, D> PartialEq<Q<A, B, C>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn eq(&self, other: &Q<A, B, C>) -> bool {
        self == other
    }
}

impl<const A: u8, B, C> PartialEq<B> for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn eq(&self, other: &B) -> bool {
        &self.n == other
    }
}

impl<const A: u8, B, C> PartialEq<f32> for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        &self.to_f32().unwrap_or_default() == other
    }
}

impl<const A: u8, B, C> PartialEq<f64> for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        &self.n.to_f64().unwrap_or_default() == other
    }
}