use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Rad<const A: u8, B> = Q<A, B, RadMode>;
            $(
                pub type [< Rad $n >]<T> = Q<$n, T, RadMode>;
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

impl<const A: u8, B, C> TryFrom<Q<A, B, DegMode, C>> for Q<A, B, RadMode, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Q<A, B, DegMode, C>) -> ::core::result::Result<Self, Self::Error> {
        value.to_rad()
    }
}

impl<const A: u8, B, C> Q<A, B, RadMode, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        let ret: B = self.n;
        let ret: B = C::tan(ret)?;
        let ret: Q<A, B, DefaultMode, C> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        let ret: B = self.n;
        let ret: B = C::sin(ret)?;
        let ret: Q<A, B, DefaultMode, C> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        let ret: B = self.n;
        let ret: B = C::cos(ret)?;
        let ret: Q<A, B, DefaultMode, C> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn to_deg(self) -> Result<Q<A, B, DegMode, C>> {
        let ret: B = self.n;
        let ret: B = C::to_deg(ret)?;
        let ret: Q<A, B, DegMode, C> = ret.into();
        Ok(ret)
    }
}

#[test]
fn test_sin() {
    let angle: Deg2<i32> = 16_00.into();
    let angle: Q2<i32> = angle.sin().unwrap();
    let angle_ok: Q2<i32> = 1_00.into();
    assert_eq!(angle, angle_ok);
}

#[test]
fn test_cos() {
    let angle: Deg2<i32> = 2_00.into();
    let angle: Q2<i32> = angle.cos().unwrap();
    let angle_ok: Q2<i32> = 1_00.into();
    assert_eq!(angle, angle_ok);
}