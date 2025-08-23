use super::*;

::modwire::expose!(
    pub alias
);

type Precision = u8;

#[derive(Clone)]
pub struct Point4D<const A: Precision, B, C, D = q::DefaultEngine>
where
    B: Int,
    C: Timestamp,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub timestamp: timestamp::Timestamp<C>,
    pub x: q::Q<A, B, q::DefaultMode, D>,
    pub y: q::Q<A, B, q::DefaultMode, D>,
    pub z: q::Q<A, B, q::DefaultMode, D>
}

impl<const A: Precision, B, C, D> Default for Point4D<A, B, C, D>
where
    B: Int,
    C: Timestamp,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn default() -> Self {
        Self {
            timestamp: C::AS_0.into(),
            x: B::AS_0.into(),
            y: B::AS_0.into(),
            z: B::AS_0.into()
        }
    }
}

impl<const A: Precision, B, C, D> From<(C, B, B, B)> for Point4D<A, B, C, D>
where
    B: Int,
    C: Timestamp,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: (C, B, B, B)) -> Self {
        let timestamp: C = value.0;
        let timestamp: timestamp::Timestamp<_> = timestamp.into();
        let x: B = value.1;
        let x: q::Q<A, B, q::DefaultMode, D> = x.into();
        let y: B = value.2;
        let y: q::Q<A, B, q::DefaultMode, D> = y.into();
        let z: B = value.3;
        let z: q::Q<A, B, q::DefaultMode, D> = z.into();
        Self {
            timestamp,
            x,
            y,
            z
        }
    }
}

impl<const A: Precision, B, C, D> From<(B, B, B)> for Point4D<A, B, C, D>
where
    B: Int,
    C: Timestamp,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: (B, B, B)) -> Self {
        let timestamp: timestamp::Timestamp<C> = C::AS_0.into();
        let x: B = value.0;
        let x: q::Q<A, B, q::DefaultMode, D> = x.into();
        let y: B = value.1;
        let y: q::Q<A, B, q::DefaultMode, D> = y.into();
        let z: B = value.2;
        let z: q::Q<A, B, q::DefaultMode, D> = z.into();
        Self {
            timestamp,
            x,
            y,
            z
        }
    }
}