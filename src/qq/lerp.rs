use super::*;

impl<const A: u8, B> Q<A, B>
where
    B: ops::Int {
    pub fn lerp(
        self, 
        rhs: impl Into<Self>, 
        pct: impl Into<Self>
    ) -> Result<Self> {
        let x: Self = self;
        let x: B = x.0;
        let y: Self = rhs.into();
        let y: B = y.0;
        let pct: Q<A, B> = pct.into();
        let pct: B = pct.0;
        let n: B = lerp(x, y, pct)?;
        let n: Self = n.into();
        Ok(n)
    }
}

#[inline]
fn lerp<const A: u8, B>(x: B, y: B, t: B) -> Result<B>
where
    B: ops::Int {
    let d: B = sub(y, x)?;
    let s: B = muldiv(d, t, scale::<A, _>())?;
    let n: B = add(x, s)?;
    Ok(n)
}