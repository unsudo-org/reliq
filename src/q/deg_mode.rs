use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Deg<const A: Precision, B> = Q<A, B, DegMode>;
            $(
                pub type [< Deg $n >]<T> = Q<$n, T, DegMode>;
            )*
        );
    };
}

ty!(
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct DegMode;

impl<const A: Precision, B, C> TryFrom<Q<A, B, RadMode, C>> for Q<A, B, DegMode, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Q<A, B, RadMode, C>) -> ::core::result::Result<Self, Self::Error> {
        value.to_deg()
    }
}

impl<const A: Precision, B, C> Q<A, B, DegMode, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        self.to_rad()?.tan()
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        self.to_rad()?.sin()
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        self.to_rad()?.cos()
    }

    #[inline]
    pub fn to_rad(self) -> Result<Q<A, B, RadMode, C>> {
        let ret: B = self.n;
        let ret: B = C::to_rad(ret)?;
        let ret: Q<A, B, RadMode, C> = ret.into();
        Ok(ret)
    }
}

#[cfg(test)]
#[allow(clippy::zero_prefixed_literal)]
mod test {
    use super::*;

    #[::rstest::rstest]
    #[case(25_00.into(), 0_46.into())]
    fn tan(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = angle.tan().unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(25_00.into(), 0_42.into())]
    fn sin(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = angle.sin().unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into())]
    fn cos(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = angle.cos().unwrap();
        assert_eq!(ret, expected);
    }
}