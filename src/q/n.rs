use super::*;

/// `T` must be able to support `180`.
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
pub fn n0<T>() -> T
where
    T: num::Int,
    (): N<T> {
    T::AS_0
}

#[inline]
pub fn n100<T>() -> T
where
    T: num::Int,
    (): N<T> {
    let ret: u8 = 100;
    let ret: T = unsafe {
        ret.try_into().unwrap_unchecked()
    };
    ret
}

#[inline]
pub fn n180<T>() -> T
where
    T: num::Int,
    (): N<T> {
    let ret: u8 = 180;
    let ret: T = unsafe {
        ret.try_into().unwrap_unchecked()
    };
    ret
}