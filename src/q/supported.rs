use super::*;

macro_rules! precision_whitelist {
    ($($n:literal)*) => {
        $(
            impl SupportedPrecision<$n> for () {}
        )*
    };
}

macro_rules! int_whitelist {
    ($($ty:ident)*) => {
        $(
            impl SupportedInt<$ty> for () {}
        )*
    };
}

macro_rules! whitelist {
    ($($n:literal $ty:ty)*) => {
        $(
            impl Supported<$n, $ty> for () {}
        )*
    };
}

/// `A` is a valid precision.
pub trait SupportedPrecision<const T: Precision> {}

/// `T` must be able to support `180`.
pub trait SupportedInt<T> {}

/// * `A` & `B` can safely hold `10^A`.
/// * `A` & `B` can safely hold `π`.
pub trait Supported<const A: Precision, B> {}

precision_whitelist!(
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

int_whitelist!(
    u8 u16 u32 u64 u128 usize
       i16 i32 i64 i128 isize
);

whitelist!(
    0 u8
    1 u8
    2 u8

    0 u16
    1 u16
    2 u16
    3 u16
    4 u16

    0 u32
    1 u32
    2 u32
    3 u32
    4 u32
    5 u32
    6 u32
    7 u32
    8 u32
    9 u32

    0 u64
    1 u64
    2 u64
    3 u64
    4 u64
    5 u64
    6 u64
    7 u64
    8 u64
    9 u64
    10 u64
    11 u64
    12 u64
    13 u64
    14 u64
    15 u64
    16 u64
    17 u64
    18 u64
    19 u64

    1 u128
    2 u128
    3 u128
    4 u128
    5 u128
    6 u128
    7 u128
    8 u128
    9 u128
    10 u128
    11 u128
    12 u128
    13 u128
    14 u128
    15 u128
    16 u128
    17 u128
    18 u128
    19 u128
    20 u128
    21 u128
    22 u128
    23 u128
    24 u128
    25 u128
    26 u128
    27 u128
    28 u128
    29 u128
    30 u128
    31 u128
    32 u128
    33 u128
    34 u128
    35 u128
    36 u128
    37 u128

    // platform-dependent, 32-bit for safety
    1 usize
    2 usize
    3 usize
    4 usize
    5 usize
    6 usize
    7 usize
    8 usize
    9 usize

    // i8 does not support `180`.

    1 i16
    2 i16
    3 i16
    4 i16

    1 i32
    2 i32
    3 i32
    4 i32
    5 i32
    6 i32
    7 i32
    8 i32
    9 i32

    1 i64
    2 i64
    3 i64
    4 i64
    5 i64
    6 i64
    7 i64
    8 i64
    9 i64
    10 i64
    11 i64
    12 i64
    13 i64
    14 i64
    15 i64
    16 i64
    17 i64
    18 i64

    1 i128
    2 i128
    3 i128
    4 i128
    5 i128
    6 i128
    7 i128
    8 i128
    9 i128
    10 i128
    11 i128
    12 i128
    13 i128
    14 i128
    15 i128
    16 i128
    17 i128
    18 i128
    19 i128
    20 i128
    21 i128
    22 i128
    23 i128
    24 i128
    25 i128
    26 i128
    27 i128
    28 i128
    29 i128
    30 i128
    31 i128
    32 i128
    33 i128
    34 i128
    35 i128
    36 i128
    37 i128

    // platform-dependent, 32-bit for safety
    1 isize
    2 isize
    3 isize
    4 isize
    5 isize
    6 isize
    7 isize
    8 isize
    9 isize
);