use super::*;

impl<const A: u8, B> Q<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn round_up(self) -> Self {
        let n: B = self.0;
        let scale: B = scale::<A, _>();
        if n % scale == B::AS_0 {
            let n: Self = n.into();
            return n
        }
        if n >= B::AS_0 {
            let n: B = n / scale;
            let n: B = n + B::AS_1;
            let n: B = n * scale;
            let n: Self = n.into();
            return n
        }
        let n: B = n / scale;
        let n: B = n * scale;
        let n: Self = n.into();
        n
    }

    #[inline]
    pub fn round_down(self) -> Self {
        let n: B = self.0;
        let scale: B = scale::<A, _>();
        if n % scale == B::AS_0 {
            let n: Self = n.into();
            return n
        }
        if n >= B::AS_0 {
            let n: B = n / scale;
            let n: B = n * scale;
            let n: Self = n.into();
            return n
        }
        let n: B = n / scale;
        let n: B = n - B::AS_1;
        let n: B = n * scale;
        let n: Self = n.into();
        n
    }

    #[inline]
    pub fn round_towards_zero(self) -> Self {
        let scale: B = scale::<A, _>();
        let n: B = self.0;
        let n: B = n / scale;
        let n: B = n * scale;
        let n: Self = n.into();
        n
    }

    #[inline]
    pub fn round_away_from_zero(self) -> Self {
        let n: B = self.0;
        let scale: B = scale::<A, _>();
        if n % scale == B::AS_0 {
            let n: Self = n.into();
            return n
        }
        if n < B::AS_0 {
            let n: B = n / scale;
            let n: B = n * scale;
            let n: B = n - scale;
            let n: Self = n.into();
            return n
        }
        let n: B = n / scale;
        let n: B = n * scale;
        let n: B = n + scale;
        let n: Self = n.into();
        n
    }
}