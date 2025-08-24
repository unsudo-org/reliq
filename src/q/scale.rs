use super::*;

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
pub(super) fn scale<const A: Precision, B>() -> B 
where 
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    match (B::SIGNED, B::BITS_AS_U128, A) {
        (_, _, 0) => B::AS_3,
        (true, 8, 1..=1)
        | (true, 16, 1..=4)
        | (true, 32, 1..=9)
        | (true, 64, 1..=19)
        | (true, 128, 1..=37)
        | (false, 8, 1..=2)
        | (false, 16, 1..=4)
        | (false, 32, 1..=9)
        | (false, 64, 1..=19)
        | (false, 128, 1..=37) => {
            unsafe {
                look_up::<A>().try_into().unwrap_unchecked()
            }
        },
        _ => {
            unsafe {
                ::core::hint::unreachable_unchecked()
            }
        }
    }
}

#[inline]
const fn look_up<const T: Precision>() -> u128 {
    LOOK_UP[(T - 1) as usize]
}