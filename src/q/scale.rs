use super::*;

pub trait ScaleCompatible<const A: u8, B> {}

// u8: 10^2
impl ScaleCompatible<1, u8> for () {}
impl ScaleCompatible<2, u8> for () {}

// u16: 10^4
impl ScaleCompatible<1, u16> for () {}
impl ScaleCompatible<2, u16> for () {}
impl ScaleCompatible<3, u16> for () {}
impl ScaleCompatible<4, u16> for () {}

// u32: 10^9
impl ScaleCompatible<1, u32> for () {}
impl ScaleCompatible<2, u32> for () {}
impl ScaleCompatible<3, u32> for () {}
impl ScaleCompatible<4, u32> for () {}
impl ScaleCompatible<5, u32> for () {}
impl ScaleCompatible<6, u32> for () {}
impl ScaleCompatible<7, u32> for () {}
impl ScaleCompatible<8, u32> for () {}
impl ScaleCompatible<9, u32> for () {}

// u64: 10^19
impl ScaleCompatible<1, u64> for () {}
impl ScaleCompatible<2, u64> for () {}
impl ScaleCompatible<3, u64> for () {}
impl ScaleCompatible<4, u64> for () {}
impl ScaleCompatible<5, u64> for () {}
impl ScaleCompatible<6, u64> for () {}
impl ScaleCompatible<7, u64> for () {}
impl ScaleCompatible<8, u64> for () {}
impl ScaleCompatible<9, u64> for () {}
impl ScaleCompatible<10, u64> for () {}
impl ScaleCompatible<11, u64> for () {}
impl ScaleCompatible<12, u64> for () {}
impl ScaleCompatible<13, u64> for () {}
impl ScaleCompatible<14, u64> for () {}
impl ScaleCompatible<15, u64> for () {}
impl ScaleCompatible<16, u64> for () {}
impl ScaleCompatible<17, u64> for () {}
impl ScaleCompatible<18, u64> for () {}
impl ScaleCompatible<19, u64> for () {}

// u128: 10^38
impl ScaleCompatible<1, u128> for () {}
impl ScaleCompatible<2, u128> for () {}
impl ScaleCompatible<3, u128> for () {}
impl ScaleCompatible<4, u128> for () {}
impl ScaleCompatible<5, u128> for () {}
impl ScaleCompatible<6, u128> for () {}
impl ScaleCompatible<7, u128> for () {}
impl ScaleCompatible<8, u128> for () {}
impl ScaleCompatible<9, u128> for () {}
impl ScaleCompatible<10, u128> for () {}
impl ScaleCompatible<11, u128> for () {}
impl ScaleCompatible<12, u128> for () {}
impl ScaleCompatible<13, u128> for () {}
impl ScaleCompatible<14, u128> for () {}
impl ScaleCompatible<15, u128> for () {}
impl ScaleCompatible<16, u128> for () {}
impl ScaleCompatible<17, u128> for () {}
impl ScaleCompatible<18, u128> for () {}
impl ScaleCompatible<19, u128> for () {}
impl ScaleCompatible<20, u128> for () {}
impl ScaleCompatible<21, u128> for () {}
impl ScaleCompatible<22, u128> for () {}
impl ScaleCompatible<23, u128> for () {}
impl ScaleCompatible<24, u128> for () {}
impl ScaleCompatible<25, u128> for () {}
impl ScaleCompatible<26, u128> for () {}
impl ScaleCompatible<27, u128> for () {}
impl ScaleCompatible<28, u128> for () {}
impl ScaleCompatible<29, u128> for () {}
impl ScaleCompatible<30, u128> for () {}
impl ScaleCompatible<31, u128> for () {}
impl ScaleCompatible<32, u128> for () {}
impl ScaleCompatible<33, u128> for () {}
impl ScaleCompatible<34, u128> for () {}
impl ScaleCompatible<35, u128> for () {}
impl ScaleCompatible<36, u128> for () {}
impl ScaleCompatible<37, u128> for () {}
impl ScaleCompatible<38, u128> for () {}

// usize: 10^9
// platform-dependent, but 32-bit for safety
impl ScaleCompatible<1, usize> for () {}
impl ScaleCompatible<2, usize> for () {}
impl ScaleCompatible<3, usize> for () {}
impl ScaleCompatible<4, usize> for () {}
impl ScaleCompatible<5, usize> for () {}
impl ScaleCompatible<6, usize> for () {}
impl ScaleCompatible<7, usize> for () {}
impl ScaleCompatible<8, usize> for () {}
impl ScaleCompatible<9, usize> for () {}

// i8 - 10^1
impl ScaleCompatible<1, i8> for () {}

// i16 - 10^4
impl ScaleCompatible<1, i16> for () {}
impl ScaleCompatible<2, i16> for () {}
impl ScaleCompatible<3, i16> for () {}
impl ScaleCompatible<4, i16> for () {}

// i32 - 10^9
impl ScaleCompatible<1, i32> for () {}
impl ScaleCompatible<2, i32> for () {}
impl ScaleCompatible<3, i32> for () {}
impl ScaleCompatible<4, i32> for () {}
impl ScaleCompatible<5, i32> for () {}
impl ScaleCompatible<6, i32> for () {}
impl ScaleCompatible<7, i32> for () {}
impl ScaleCompatible<8, i32> for () {}
impl ScaleCompatible<9, i32> for () {}

// i64 - 10^18
impl ScaleCompatible<1, i64> for () {}
impl ScaleCompatible<2, i64> for () {}
impl ScaleCompatible<3, i64> for () {}
impl ScaleCompatible<4, i64> for () {}
impl ScaleCompatible<5, i64> for () {}
impl ScaleCompatible<6, i64> for () {}
impl ScaleCompatible<7, i64> for () {}
impl ScaleCompatible<8, i64> for () {}
impl ScaleCompatible<9, i64> for () {}
impl ScaleCompatible<10, i64> for () {}
impl ScaleCompatible<11, i64> for () {}
impl ScaleCompatible<12, i64> for () {}
impl ScaleCompatible<13, i64> for () {}
impl ScaleCompatible<14, i64> for () {}
impl ScaleCompatible<15, i64> for () {}
impl ScaleCompatible<16, i64> for () {}
impl ScaleCompatible<17, i64> for () {}
impl ScaleCompatible<18, i64> for () {}

// i128 - 10^38
impl ScaleCompatible<1, i128> for () {}
impl ScaleCompatible<2, i128> for () {}
impl ScaleCompatible<3, i128> for () {}
impl ScaleCompatible<4, i128> for () {}
impl ScaleCompatible<5, i128> for () {}
impl ScaleCompatible<6, i128> for () {}
impl ScaleCompatible<7, i128> for () {}
impl ScaleCompatible<8, i128> for () {}
impl ScaleCompatible<9, i128> for () {}
impl ScaleCompatible<10, i128> for () {}
impl ScaleCompatible<11, i128> for () {}
impl ScaleCompatible<12, i128> for () {}
impl ScaleCompatible<13, i128> for () {}
impl ScaleCompatible<14, i128> for () {}
impl ScaleCompatible<15, i128> for () {}
impl ScaleCompatible<16, i128> for () {}
impl ScaleCompatible<17, i128> for () {}
impl ScaleCompatible<18, i128> for () {}
impl ScaleCompatible<19, i128> for () {}
impl ScaleCompatible<20, i128> for () {}
impl ScaleCompatible<21, i128> for () {}
impl ScaleCompatible<22, i128> for () {}
impl ScaleCompatible<23, i128> for () {}
impl ScaleCompatible<24, i128> for () {}
impl ScaleCompatible<25, i128> for () {}
impl ScaleCompatible<26, i128> for () {}
impl ScaleCompatible<27, i128> for () {}
impl ScaleCompatible<28, i128> for () {}
impl ScaleCompatible<29, i128> for () {}
impl ScaleCompatible<30, i128> for () {}
impl ScaleCompatible<31, i128> for () {}
impl ScaleCompatible<32, i128> for () {}
impl ScaleCompatible<33, i128> for () {}
impl ScaleCompatible<34, i128> for () {}
impl ScaleCompatible<35, i128> for () {}
impl ScaleCompatible<36, i128> for () {}
impl ScaleCompatible<37, i128> for () {}
impl ScaleCompatible<38, i128> for () {}

// platform-dependent, but 32-bit for safety
impl ScaleCompatible<1, isize> for () {}
impl ScaleCompatible<2, isize> for () {}
impl ScaleCompatible<3, isize> for () {}
impl ScaleCompatible<4, isize> for () {}
impl ScaleCompatible<5, isize> for () {}
impl ScaleCompatible<6, isize> for () {}
impl ScaleCompatible<7, isize> for () {}
impl ScaleCompatible<8, isize> for () {}
impl ScaleCompatible<9, isize> for () {}

const LOOK_UP: [u128; 38] = [
    10u128.pow(1),
    10u128.pow(2),
    10u128.pow(3),
    10u128.pow(4),
    10u128.pow(5),
    10u128.pow(6),
    10u128.pow(7),
    10u128.pow(8),
    10u128.pow(9),
    10u128.pow(10),
    10u128.pow(11),
    10u128.pow(12),
    10u128.pow(13),
    10u128.pow(14),
    10u128.pow(15),
    10u128.pow(16),
    10u128.pow(17),
    10u128.pow(18),
    10u128.pow(19),
    10u128.pow(20),
    10u128.pow(21),
    10u128.pow(22),
    10u128.pow(23),
    10u128.pow(24),
    10u128.pow(25),
    10u128.pow(26),
    10u128.pow(27),
    10u128.pow(28),
    10u128.pow(29),
    10u128.pow(30),
    10u128.pow(31),
    10u128.pow(32),
    10u128.pow(33),
    10u128.pow(34),
    10u128.pow(35),
    10u128.pow(36),
    10u128.pow(37),
    10u128.pow(38)
];

#[inline]
pub fn scale<const A: u8, B>() -> B 
where 
    B: num::Int,
    // out of bounds guard is covered by `Precision<A>`
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B> {
    unsafe {
        LOOK_UP[(A - 1) as usize].try_into().unwrap_unchecked()
    }
}