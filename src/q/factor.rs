use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Factor<const A: u8, B, C = DefaultEngine> = Q<A, B, FactorMode, C>;
            $(
                pub type [< Factor $n >]<A = usize, B = DefaultEngine> = Q<$n, A, FactorMode, B>;
            )*
        );
    };
}

ty!(
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

/// A factor represents an alternative form to a `Percentage`,
/// where anything less than `1.0` represents a negative `%`,
/// and anything more than `1.0` represents a positive `%`.
/// 
/// # Example
/// * 1.0 -> 0%
/// * 2.0 -> 100%
/// * 0.5 -> -50%
/// * 0.25 -> -75%
/// 
/// # Conversion: From
/// * `Unit`
/// * `Percentage`
#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct FactorMode;

impl<const A: u8, B, C> Factor<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    pub fn to_unit(self) -> Unit<A, B, C> {
        self.into()
    }

    pub fn to_percentage(self) -> Percentage<A, B, C> {
        self.into()
    }
}

impl<const A: u8, B, C> From<B> for Factor<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: B) -> Self {
        Self {
            n: value,
            mode: ::core::marker::PhantomData,
            engine: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Unit<A, B, C>> for Factor<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Unit<A, B, C>) -> Self {
        Self {
            n: value.n,
            mode: ::core::marker::PhantomData,
            engine: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Percentage<A, B, C>> for Factor<A, B, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Percentage<A, B, C>) -> Self {
        
    }
}