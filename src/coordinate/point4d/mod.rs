use super::*;

type Precision = u8;

#[derive(Clone)]
pub struct Point4D<const A: Precision, B, C = q::DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub timestamp: ::core::time::Duration,
    pub x: q::Q<A, B, q::DefaultMode, C>,
    pub y: q::Q<A, B, q::DefaultMode, C>,
    pub z: q::Q<A, B, q::DefaultMode, C>
}

impl<const A: Precision, B, C> Default for Point4D<A, B, C>
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

impl<const A: Precision, B, C> From<(::core::time::Duration, B, B, B)> for Point4D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: (::core::time::Duration, B, B, B)) -> Self {
        let timestamp: ::core::time::Duration = value.0;
        let x: B = value.1;
        let x: q::Q<A, B, q::DefaultMode, C> = x.into();
        let y: B = value.2;
        let y: q::Q<A, B, q::DefaultMode, C> = y.into();
        let z: B = value.3;
        let z: q::Q<A, B, q::DefaultMode, C> = z.into();
        Self {
            timestamp,
            x,
            y,
            z
        }
    }
}

impl<const A: Precision, B, C> From<(B, B, B)> for Point4D<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: (B, B, B)) -> Self {
        let x: B = value.0;
        let x: q::Q<A, B, q::DefaultMode, C> = x.into();
        let y: B = value.1;
        let y: q::Q<A, B, q::DefaultMode, C> = y.into();
        let z: B = value.2;
        let z: q::Q<A, B, q::DefaultMode, C> = z.into();
        Self {
            timestamp: ::core::time::Duration::default(),
            x,
            y,
            z
        }
    }
}