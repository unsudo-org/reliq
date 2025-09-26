use super::*;

impl<const A: u8, B, C> Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn round_up(self) -> Self {
        let n: B = self.n;
        let scale: B = scale::<A, _>();
        if n % scale == B::AS_0 {
            let n: Self = Self::from_raw(n);
            return n
        }
        if n >= B::AS_0 {
            let n: B = n / scale;
            let n: B = n + B::AS_1;
            let n: B = n * scale;
            let n: Self = Self::from_raw(n);
            return n
        }
        let n: B = n / scale;
        let n: B = n * scale;
        let n: Self = Self::from_raw(n);
        n
    }

    #[inline]
    pub fn round_down(self) -> Self {
        let n: B = self.n;
        let scale: B = scale::<A, _>();
        if n % scale == B::AS_0 {
            let n: Self = Self::from_raw(n);
            return n
        }
        if n >= B::AS_0 {
            let n: B = n / scale;
            let n: B = n * scale;
            let n: Self = Self::from_raw(n);
            return n
        }
        let n: B = n / scale;
        let n: B = n - B::AS_1;
        let n: B = n * scale;
        let n: Self = Self::from_raw(n);
        n
    }

    #[inline]
    pub fn round_towards_zero(self) -> Self {
        let scale: B = scale::<A, _>();
        let n: B = self.n;
        let n: B = n / scale;
        let n: B = n * scale;
        let n: Self = Self {
            n,
            m_0: ::core::marker::PhantomData
        };
        n
    }

    #[inline]
    pub fn round_away_from_zero(self) -> Self {
        let n: B = self.n;
        let scale: B = scale::<A, _>();
        if n % scale == B::AS_0 {
            let n: Self = Self::from_raw(n);
            return n
        }
        if n < B::AS_0 {
            let n: B = n / scale;
            let n: B = n * scale;
            let n: B = n - scale;
            let n: Self = Self::from_raw(n);
            return n
        }
        let n: B = n / scale;
        let n: B = n * scale;
        let n: B = n + scale;
        let n: Self = Self::from_raw(n);
        n
    }
}