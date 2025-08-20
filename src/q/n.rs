use super::*;

/// `T` must be able to support `180`.
pub trait N<T> 
where
    T: num::Int {}

impl N<u8> for () {}
impl N<u16> for () {}
impl N<u32> for () {}
impl N<u64> for () {}
impl N<u128> for () {}
impl N<usize> for () {}
impl N<i16> for () {}
impl N<i32> for () {}
impl N<i64> for () {}
impl N<i128> for () {}
impl N<isize> for () {}