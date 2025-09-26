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
        E: Into<Percentage<A, B>> {
        let x: Self = self;
        let x: B = x.n;
        let y: Self = rhs.into();
        let y: B = y.n;
        let percentage: Percentage<A, B> = percentage.into();
        let percentage: B = percentage.n;
        let n: B = lerp(x, y, percentage)?;
        let n: Self = n.into();
        Ok(n)
    }
}

#[inline]
fn lerp<const A: u8, B>(x: B, y: B, t: B) -> Result<B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    let d: B = sub(y, x)?;
    let s: B = muldiv(d, t, scale::<A, _>())?;
    let n: B = add(x, s)?;
    Ok(n)
}