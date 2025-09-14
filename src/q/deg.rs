use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Deg<const A: u8 = 2, B = usize, C = DefaultEngine> = Q<A, B, DegMode, C>;
            $(
                pub type [< Deg $n >]<A = usize, B = DefaultEngine> = Q<$n, A, DegMode, B>;
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

impl<const A: u8, B, C> Deg<A, B, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Unit<A, B, C>> {
        self.to_rad()?.tan()
    }

    #[inline]
    pub fn sin(self) -> Result<Unit<A, B, C>> {
        self.to_rad()?.sin()
    }

    #[inline]
    pub fn cos(self) -> Result<Unit<A, B, C>> {
        self.to_rad()?.cos()
    }

    #[inline]
    pub fn to_rad(self) -> Result<Rad<A, B, C>> {
        let ret: B = self.n;
        let ret: B = C::to_rad(ret)?;
        let ret: Rad<A, B, C> = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C> TryFrom<Rad<A, B, C>> for Deg<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Rad<A, B, C>) -> ::core::result::Result<Self, Self::Error> {
        value.to_deg()
    }
}

#[cfg(test)]
#[allow(clippy::zero_prefixed_literal)]
mod test {
    use super::*;

    #[::rstest::rstest]
    #[case(25_00.into(), 0_46.into())]
    fn tan(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Unit = angle.tan().unwrap();
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