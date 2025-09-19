#![allow(clippy::zero_prefixed_literal)]
#![allow(clippy::let_with_type_underscore)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod array;
pub mod color;
pub mod fail_safe;
pub mod lossy;
pub mod map;
pub mod math;
pub mod ops;
pub mod point;
pub mod q;
pub mod utf8;


fn hello(n: q::Unit2, y: q::Unit2) {
    let x = n + y - y + n;
    x.into_result().unwrap();
}