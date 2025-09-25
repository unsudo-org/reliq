use super::*;

impl<const A: u8, B, C> Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn cast<const D: u8>(self) -> Result<Q<D, B, C>> 
    where
        (): SupportedPrecision<D>,
        (): Supported<D, B> {
        let n: B = self.n;
        let old_scale: B = scale::<A, _>();
        let new_scale: B = scale::<D, _>();
        let n: B = muldiv(n, new_scale, old_scale)?;
        let n: Q<D, B, C> = n.into();
        Ok(n)
    }
}