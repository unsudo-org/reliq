use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Chance<const A: u8, B, C = DefaultEngine> = Q<A, B, ChanceMode, C>;
            $(
                pub type [< Chance $n >]<A, B = DefaultEngine> = Q<$n, A, ChanceMode, B>;
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
pub struct ChanceMode;

impl<const A: u8, B, C> From<Unit<A, B, C>> for Chance<A, B, C>
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
            mode: ::core::marker::PhantomData,
            engine: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Percentage<A, B, C>> for Chance<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Percentage<A, B, C>) -> Self {
        Self {
            n: value.n,
            mode: ::core::marker::PhantomData,
            engine: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Factor<A, B, C>> for Chance<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Factor<A, B, C>) -> Self {
        let ret: Percentage<A, B, C> = value.into();
        let ret: Self = ret.into();
        ret
    }
}