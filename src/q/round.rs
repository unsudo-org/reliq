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
        let ret: B = D::round_up(n);
        let ret: Self = ret.into();
        ret
    }

    #[inline]
    pub fn round_down(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_down(n);
        let ret: Self = ret.into();
        ret
    }

    #[inline]
    pub fn round_towards_zero(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_towards_zero(n);
        let ret: Self = ret.into();
        ret
    }

    #[inline]
    pub fn round_away_from_zero(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_away_from_zero(n);
        let ret: Self = ret.into();
        ret
    }
}