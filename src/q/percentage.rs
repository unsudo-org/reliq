use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Percentage<const A: u8, B, C = DefaultEngine> = Q<A, B, PercentageMode, C>;
            $(
                pub type [< Percentage $n >]<A, B = DefaultEngine> = Percentage<$n, A, B>;
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

// cannot have less than -100, but can be higher.
#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct PercentageMode;

impl<const A: u8, B, C> From<Unit<A, B, C>> for Percentage<A, B, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Unit<A, B, C>) -> Self {
        Self {
            n: value.n,
            engine: ::core::marker::PhantomData,
            mode: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> TryFrom<Factor<A, B, C>> for Percentage<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B>,
    (): Supported<1, B> {
    type Error = Error;
    
    fn try_from(value: Factor<A, B, C>) -> ::core::result::Result<Self, Self::Error> {
        let n: Unit<_, _, _> = value.into();
        if n >= as_1() {
            let ret: Factor<_, _, _> = ((n * as_100())? - as_100())?;
            let ret: B = ret.n;
            let ret: Self = ret.into();
            return Ok(ret)
        }
        if !B::SIGNED {
            return Err(Error::Underflow)
        }
        let n: Factor<_, _, _> = (as_1() - n)?;
        let n = (n * as_100())?;
        Ok()
    }
}