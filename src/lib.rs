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


fn scale_display(w: q::Unit2<u32>, f: q::Deg2<u32>) {
    if w == f {
        w.round_away_from_zero();
        w.round_down();
        
        f.cos().unwrap();
    }
}