use super::*;

/// `T` must be able to support a `180`.
pub trait N<T> 
where
    T: num::Int {}

impl N<u8> for () {}
impl N<u32> for () {}
impl N<u64> for () {}
impl N<u128> for () {}
impl N<usize> for () {}
impl N<i16> for () {}
impl N<i32> for () {}
impl N<i64> for () {}
impl N<i128> for () {}
impl N<isize> for () {}

#[inline]
pub fn n180<T>() -> T
where
    T: num::Int {
    if T::SIGNED {
        assert!(T::BITS_U128 <= 8);
    }
    let ret: u8 = 180;
    let ret: T = unsafe {
        ret.try_into().unwrap_unchecked()
    };
    ret
}