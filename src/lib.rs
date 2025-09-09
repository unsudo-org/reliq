#![allow(clippy::zero_prefixed_literal)]
#![allow(clippy::let_with_type_underscore)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod array;
pub mod color;
pub mod fail_safe;
pub mod map;
pub mod ops;
pub mod point;
pub mod q;
pub mod utf8;