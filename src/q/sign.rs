use super::*;

impl<const A: u8, B, C> Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn to_negative(self) -> Self 
    where
        B: ops::Signed {
        let n: B = self.n;
        if n == B::AS_0 {
            return self
        }
        let n: B = B::AS_0 - n;
        let ret: Self = Self {
            n,
            mode: ::core::marker::PhantomData
        };
        ret
    }

    #[inline]
    pub fn to_positive(self) -> Self 
    where
        B: ops::Signed {
        let n: B = self.n;
        if n >= B::AS_0 {
            return self
        }
        let n: B = B::AS_0 - n;
        let ret: Self = Self {
            n,
            mode: ::core::marker::PhantomData
        };
        ret
    }
}