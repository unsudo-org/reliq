use super::*;

pub use super::Result;
pub use super::Error;

type IntTupleWithTimetamp<T> = (::core::time::Duration, T, T, T);
type IntTuple<T> = (T, T, T);
type Q<const A: u8, B, C> = q::Q<A, B, q::DefaultMode, C>;

#[derive(Clone)]
pub struct Point4D<const A: u8, B, C = q::DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub timestamp: ::core::time::Duration,
    pub x: Q<A, B, C>,
    pub y: Q<A, B, C>,
    pub z: Q<A, B, C>
}

impl<const A: u8, B, C> Default for Point4D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn default() -> Self {
        Self {
            timestamp: ::core::time::Duration::default(),
            x: B::AS_0.into(),
            y: B::AS_0.into(),
            z: B::AS_0.into()
        }
    }
}

impl<const A: u8, B, C> From<IntTupleWithTimetamp<B>> for Point4D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: IntTupleWithTimetamp<B>) -> Self {
        let timestamp: ::core::time::Duration = value.0;
        let x: B = value.1;
        let x: Q<A, B, C> = x.into();
        let y: B = value.2;
        let y: Q<A, B, C> = y.into();
        let z: B = value.3;
        let z: Q<A, B, C> = z.into();
        Self {
            timestamp,
            x,
            y,
            z
        }
    }
}

impl<const A: u8, B, C> From<IntTuple<B>> for Point4D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: IntTuple<B>) -> Self {
        let x: B = value.0;
        let x: Q<A, B, C> = x.into();
        let y: B = value.1;
        let y: Q<A, B, C> = y.into();
        let z: B = value.2;
        let z: Q<A, B, C> = z.into();
        Self {
            timestamp: ::core::time::Duration::default(),
            x,
            y,
            z
        }
    }
}