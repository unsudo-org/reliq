use super::*;

pub trait PICompatible<const A: u8, B> {}

// u8 (can safely hold π * 10^2 = 314)
impl PICompatible<1, u8> for () {}
impl PICompatible<2, u8> for () {}

// u16 (π * 10^4 = 31415)
impl PICompatible<1, u16> for () {}
impl PICompatible<2, u16> for () {}
impl PICompatible<3, u16> for () {}
impl PICompatible<4, u16> for () {}

// u32 (π * 10^9 = 3141592653)
impl PICompatible<1, u32> for () {}
impl PICompatible<2, u32> for () {}
impl PICompatible<3, u32> for () {}
impl PICompatible<4, u32> for () {}
impl PICompatible<5, u32> for () {}
impl PICompatible<6, u32> for () {}
impl PICompatible<7, u32> for () {}
impl PICompatible<8, u32> for () {}
impl PICompatible<9, u32> for () {}

// u64 (π * 10^18 = 3141592653589793238)
impl PICompatible<1, u64> for () {}
impl PICompatible<2, u64> for () {}
impl PICompatible<3, u64> for () {}
impl PICompatible<4, u64> for () {}
impl PICompatible<5, u64> for () {}
impl PICompatible<6, u64> for () {}
impl PICompatible<7, u64> for () {}
impl PICompatible<8, u64> for () {}
impl PICompatible<9, u64> for () {}
impl PICompatible<10, u64> for () {}
impl PICompatible<11, u64> for () {}
impl PICompatible<12, u64> for () {}
impl PICompatible<13, u64> for () {}
impl PICompatible<14, u64> for () {}
impl PICompatible<15, u64> for () {}
impl PICompatible<16, u64> for () {}
impl PICompatible<17, u64> for () {}
impl PICompatible<18, u64> for () {}

// u128 (π * 10^38 = 31415926535897932384626433832795028841)
impl PICompatible<1, u128> for () {}
impl PICompatible<2, u128> for () {}
impl PICompatible<3, u128> for () {}
impl PICompatible<4, u128> for () {}
impl PICompatible<5, u128> for () {}
impl PICompatible<6, u128> for () {}
impl PICompatible<7, u128> for () {}
impl PICompatible<8, u128> for () {}
impl PICompatible<9, u128> for () {}
impl PICompatible<10, u128> for () {}
impl PICompatible<11, u128> for () {}
impl PICompatible<12, u128> for () {}
impl PICompatible<13, u128> for () {}
impl PICompatible<14, u128> for () {}
impl PICompatible<15, u128> for () {}
impl PICompatible<16, u128> for () {}
impl PICompatible<17, u128> for () {}
impl PICompatible<18, u128> for () {}
impl PICompatible<19, u128> for () {}
impl PICompatible<20, u128> for () {}
impl PICompatible<21, u128> for () {}
impl PICompatible<22, u128> for () {}
impl PICompatible<23, u128> for () {}
impl PICompatible<24, u128> for () {}
impl PICompatible<25, u128> for () {}
impl PICompatible<26, u128> for () {}
impl PICompatible<27, u128> for () {}
impl PICompatible<28, u128> for () {}
impl PICompatible<29, u128> for () {}
impl PICompatible<30, u128> for () {}
impl PICompatible<31, u128> for () {}
impl PICompatible<32, u128> for () {}
impl PICompatible<33, u128> for () {}
impl PICompatible<34, u128> for () {}
impl PICompatible<35, u128> for () {}
impl PICompatible<36, u128> for () {}
impl PICompatible<37, u128> for () {}

// i8 can hold up to 31 -> 10^1, 10^2 would overflow
impl PICompatible<1, i8> for () {}

// i16 can hold up to 31415 -> 10^4
impl PICompatible<1, i16> for () {}
impl PICompatible<2, i16> for () {}
impl PICompatible<3, i16> for () {}
impl PICompatible<4, i16> for () {}

// i32 can hold up to 3141592653 -> 10^9
impl PICompatible<1, i32> for () {}
impl PICompatible<2, i32> for () {}
impl PICompatible<3, i32> for () {}
impl PICompatible<4, i32> for () {}
impl PICompatible<5, i32> for () {}
impl PICompatible<6, i32> for () {}
impl PICompatible<7, i32> for () {}
impl PICompatible<8, i32> for () {}
impl PICompatible<9, i32> for () {}

// i64
impl PICompatible<1, i64> for () {}
impl PICompatible<2, i64> for () {}
impl PICompatible<3, i64> for () {}
impl PICompatible<4, i64> for () {}
impl PICompatible<5, i64> for () {}
impl PICompatible<6, i64> for () {}
impl PICompatible<7, i64> for () {}
impl PICompatible<8, i64> for () {}
impl PICompatible<9, i64> for () {}
impl PICompatible<10, i64> for () {}
impl PICompatible<11, i64> for () {}
impl PICompatible<12, i64> for () {}
impl PICompatible<13, i64> for () {}
impl PICompatible<14, i64> for () {}
impl PICompatible<15, i64> for () {}
impl PICompatible<16, i64> for () {}
impl PICompatible<17, i64> for () {}
impl PICompatible<18, i64> for () {}

// i128 can hold up to 10^38
impl PICompatible<1, i128> for () {}
impl PICompatible<2, i128> for () {}
impl PICompatible<3, i128> for () {}

// ..=36

const LOOK_UP: [u128; 37] = [
    31,
    314,
    3141,
    31415,
    314159,
    3141592,
    31415926,
    314159265,
    3141592653,
    31415926535,
    314159265358,
    3141592653589,
    31415926535897,
    314159265358979,
    3141592653589793,
    31415926535897932,
    314159265358979323,
    3141592653589793238,
    31415926535897932384,
    314159265358979323846,
    3141592653589793238462,
    31415926535897932384626,
    314159265358979323846264,
    3141592653589793238462643,
    31415926535897932384626433,
    314159265358979323846264338,
    3141592653589793238462643383,
    31415926535897932384626433832,
    314159265358979323846264338327,
    3141592653589793238462643383279,
    31415926535897932384626433832795,
    314159265358979323846264338327950,
    3141592653589793238462643383279502,
    31415926535897932384626433832795028,
    314159265358979323846264338327950288,
    3141592653589793238462643383279502884,
    31415926535897932384626433832795028841
];

#[inline(always)]
pub fn pi<const A: u8, B>() -> B
where
    B: num::Int,
    (): Precision<A>,
    (): N<B> {
    unsafe {
        LOOK_UP[(A - 1) as usize].try_into().unwrap_unchecked()
    }
}