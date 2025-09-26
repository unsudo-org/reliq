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