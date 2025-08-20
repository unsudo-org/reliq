//! # Q
//! 
//! # Example: Compile-Time Guarantee
//! ```rust
//! // Will work.
//! let x: ::reliq::q::Q28<u128>;
//! 
//! // Will fail at compile-time because `u16` cannot handle
//! // a precision of `28` and would prematurely overflow.
//! let x: ::reliq::q::Q28<u16>;
//! ```
//! 
//! # Example: Simple Usage
//! ```rust
//! pub struct Person {
//!     pub balance: ::reliq::q::Q2<u32>
//! }
//! ```
//! 
//! # Example
//! ```rust
//! // These bounds are required by `Q` to guarantee at compile
//! // time that this combination of precision and size will
//! // not prematurely overflow or underflow. Which enables it
//! // to skip these checks at runtime entirely.
//! fn add_to_balance<const A: u8, B>(amount: ::reliq::q::Q<A, B>)
//! where
//!     B: ::reliq::num::Int,
//!     B: ::reliq::num::Prim,
//!     (): ::reliq::q::Precision<A>,
//!     (): ::reliq::q::N<B>,
//!     (): ::reliq::q::ScaleCompatible<A, B>,
//!     (): ::reliq::q::PICompatible<A, B> {
//!     
//! }
//! ```
//! 
//! # Example
//! If the operation is generic over the precision but the size
//! is known before hand, then only a partial guard
//! is required.
//! ```rust
//! fn partial_compatibility<const T: u8>(count: ::reliq::q::Q<T, u16>) 
//! where
//!     (): ::reliq::q::Precision<T>,
//!     (): ::reliq::q::N<u16>,
//!     (): ::reliq::q::ScaleCompatible<T, u16>,
//!     (): ::reliq::q::PICompatible<T, u16> {
//!     
//! }
//! ```
//! 
//! # Example
//! ```rust
//! use ::reliq::q;
//! use ::reliq::num;
//! 
//! #[derive(Clone)]
//! #[derive(Copy)]
//! struct MySpecialEngine;
//! 
//! impl q::Engine for MySpecialEngine {
//!     fn muldiv<T>(x: T, y: T, z: T) -> q::Result<T>
//!     where
//!         T: num::Int,
//!         T: num::Prim,
//!         (): q::N<T> {
//!         let ret: T = x
//!             .checked_mul(y)
//!             .ok_or(q::Error::Overflow)?
//!             .checked_div(z)
//!             .ok_or(q::Error::DivisionByZero)?;
//!         Ok(ret)
//!     }
//! }
//! ```

use super::*;

::modwire::expose!(
    pub engine
    pub n
    pub pi_compatible
        pi
    pub precision
    pub scale_compatible
        scale
        trig
);

macro_rules! ty {
    ($($n:literal)*) => {
        $(
            ::paste::paste!(
                pub type [< Q $n >]<T> = Q<$n, T>;
            );
        )*
    };
}

ty!(
    1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}

/// # Where
/// * A - Precision
/// * B - Int
/// * C - Engine
#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B = usize, C = DefaultEngine>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    n: B,
    m_0: ::core::marker::PhantomData<C>
}

impl<const A: u8, B, C> Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    pub fn cast<const D: u8>(self) -> Result<Q<D, B, C>> 
    where
        (): Precision<D>,
        (): ScaleCompatible<D, B>,
        (): PICompatible<D, B> {
        let ret: B = self.n;
        let ret: B = C::cast::<A, D, _>(ret)?;
        let ret: Q<D, B, C> = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C> From<B> for Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    fn from(n: B) -> Self {
        Self {
            n,
            m_0: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> ::core::ops::Add for Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = C::add(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C> ::core::ops::Sub for Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = C::sub(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C> ::core::ops::Mul for Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = C::mul(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C> ::core::ops::Div for Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = C::div(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C> Eq for Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B>
    {}

impl<const A: u8, B, C> PartialEq for Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl<const A: u8, B, C> Ord for Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self
    where
        Self: Sized {
        if self > other {
            self
        } else {
            other
        }
    }

    #[inline]
    fn min(self, other: Self) -> Self
    where
        Self: Sized {
        if self < other {
            self
        } else {
            other
        }
    }

    #[inline]
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        let x: B = self.n;
        let y: B = other.n;
        if x > y {
            ::core::cmp::Ordering::Greater
        } else if x < y {
            ::core::cmp::Ordering::Less
        } else {
            ::core::cmp::Ordering::Equal
        }
    }
}

impl<const A: u8, B, C> PartialOrd for Q<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x >= y
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x <= y
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x > y
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x < y
    }

    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}