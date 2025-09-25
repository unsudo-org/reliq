use super::*;

impl<const A: u8, B, C> Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn lerp<D, E>(self, rhs: D, percentage: E) -> Result<Self>
    where
        D: Into<Self>,
        E: Into<Percentage<A, B, C>> {
        let x: Self = self;
        let x: B = x.n;
        let y: Self = rhs.into();
        let y: B = y.n;
        let percentage: Percentage<A, B, C> = percentage.into();
        let percentage: B = percentage.n;
        let d: B = sub(y, x)?;
        let s: B = muldiv(d, percentage, scale::<A, _>())?;
        let n: B = add(x, s)?;
        let n: Self = n.into();
        Ok(n)
    }
}