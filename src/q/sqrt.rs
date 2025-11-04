use super::*;

impl<const A: u8, B, C> Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn sqrt(self) -> Result<Self> 
    where
        B: ops::Unsigned {
        let n: B = self.n;
        if n == B::AS_0 || n == B::AS_1 {
            let ret: Self = Self {
                n,
                mode: ::core::marker::PhantomData
            };
            return Ok(ret)
        }
        let mut ret: B = n.checked_div(B::AS_2).ok_or(Error::DivisionByZero)?;
        let mut last: B;
        loop {
            last = ret;
            ret = add(ret, div::<A, _>(n, ret)?)?;
            ret = div::<A, _>(ret, B::AS_2)?;
            if ret == last || ret == last.checked_add(B::AS_1).unwrap_or(ret) {
                break
            }
        }
        let n: B = ret;
        let ret: Self = Self {
            n,
            mode: ::core::marker::PhantomData
        };
        Ok(ret)
    }
}