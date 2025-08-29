use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            $(
                pub type [< Point $n D >]<const A: u8, B, C> = Point<A, $n, B, C>;
            )*
        );
    };
}

ty!(
    2
    3
    4
    5
    6
    7
    8
    9
);