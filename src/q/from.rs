use super::*;

impl<const A: u8, B, C> From<B> for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn from(value: B) -> Self {
        let n: B = value;
        Self {
            n,
            m_0: ::core::marker::PhantomData
        }
    }
}