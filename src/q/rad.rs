use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Rad<const A: u8, B = usize, C = DefaultEngine> = Q<A, B, RadMode, C>;
            
            $(
                pub type [< Rad $n >]<A = usize, B = DefaultEngine> = Rad<$n, A, B>;
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
pub struct RadMode;

impl<const A: u8, B, C> TryFrom<Deg<A, B, C>> for Rad<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Deg<A, B, C>) -> ::core::result::Result<Self, Self::Error> {
        value.to_rad()
    }
}

impl<const A: u8, B, C> Rad<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<A, B, C>> {
        let ret: B = self.n;
        let ret: B = C::tan(ret)?;
        let ret: Unit<_, _, _> = ret.into();
        let ret: Ratio<_, _, _> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<A, B, C>> {
        let ret: B = self.n;
        let ret: B = C::sin(ret)?;
        let ret: Unit<_, _, _> = ret.into();
        let ret: Ratio<_, _, _> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<A, B, C>> {
        let ret: B = self.n;
        let ret: B = C::cos(ret)?;
        let ret: Unit<_, _, _> = ret.into();
        let ret: Ratio<_, _, _> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn to_deg(self) -> Result<Deg<_, _, _>> {
        let ret: B = self.n;
        let ret: B = C::to_deg(ret)?;
        let ret: Deg<_, _, _> = ret.into();
        Ok(ret)
    }
}

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00, 0_84)]
fn test_sin<A, B>(#[case] angle: A, #[case] ok: B) 
where
    A: Into<Rad2>,
    B: Into<Unit2> {
    let angle: Rad2 = angle.into();
    let ok: Unit2 = ok.into();
    let ret: Ratio2 = angle.sin().unwrap();
    assert_eq!(ret, ok);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00, 0_54)]
fn test_cos<A, B>(#[case] angle: A, #[case] ok: B) 
where
    A: Into<Rad2>,
    B: Into<Unit2> {
    let angle: Rad2 = angle.into();
    let ok: Unit2 = ok.into();
    let ret: Ratio2 = angle.cos().unwrap();
    assert_eq!(ret, ok);
}