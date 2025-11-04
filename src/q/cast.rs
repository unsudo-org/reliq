use super::*;

impl<const A: u8, B> Q<A, B> {
    #[inline]
    pub fn cast<const C: u8>(self) -> Result<Q<C, B>> 
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedPrecision<C>,
        (): SupportedInt<B>,
        (): Supported<A, B>,
        (): Supported<C, B> {
        let n: B = self.0;
        let old_scale: B = scale::<A, _>();
        let new_scale: B = scale::<C, _>();
        let n: B = muldiv(n, new_scale, old_scale)?;
        let n: Q<C, B> = n.into();
        Ok(n)
    }
}

#[inline]
pub(super) fn cast<const A: u8, const B: u8, C>(n: C) -> Result<C>
where
    C: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedPrecision<B>,
    (): SupportedInt<C>,
    (): Supported<A, C>,
    (): Supported<B, C> {
    let old_scale: C = scale::<A, _>();
    let new_scale: C = scale::<B, _>();
    let n: C = muldiv(n, new_scale, old_scale)?;
    Ok(n)
}