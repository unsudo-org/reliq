use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Cardinal<const A: u8, B, C = DefaultEngine> = Q<A, B, CardinalMode, C>;
            $(
                pub type [< Cardinal $n >]<A, B = DefaultEngine> = Cardinal<$n, A, B>;
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

#[derive(Clone)]
#[derive(Copy)]
pub struct CardinalMode<T>;

impl<const A: u8, B, C> Cardinal<A, B, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    
}

impl ::core::ops::Deref for Cardinal<> {
    type Target = Unit<>;

    fn deref(&self) -> &Self::Target {
        // rounds the number
    }
}