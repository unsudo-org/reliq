use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        pub type Unit<const A: u8 = 2, B = usize, C = DefaultEngine> = Q<A, B, UnitMode, C>;
        $(
            ::paste::paste!(
                pub type [< Unit $n >]<A = usize, B = DefaultEngine> = Unit<$n, A, B>;
            );
        )*
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
pub struct UnitMode;

impl<const A: u8, B, C> From<Deg<A, B, C>> for Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Deg<A, B, C>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B, C> From<Rad<A, B, C>> for Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Rad<A, B, C>) -> Self {
        value.n.into()
    }
}

impl<const A: u8, B, C> From<Percentage<A, B, C>> for Unit<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Percentage<A, B, C>) -> Self {
        value.n.into()
    }
}