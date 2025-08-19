use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        pub type Percentage<const A: u8, B> = Q<A, B, PercentageMode>;
        $(
            ::paste::paste!(
                pub type [< Percentage $n >]<T> = Q<$n, T, PercentageMode>;
            );
        )*
    };
}

ty!(
    1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

#[repr(transparent)]
pub struct PercentageMode;

impl Mode for PercentageMode {}