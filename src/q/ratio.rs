use super::*;

macro_rules! ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Ratio<const A: u8, B = usize, C = DefaultEngine> = Q<A, B, RatioMode, C>;
            $(
                pub type [< Ratio $n >]<A = usize, B = DefaultEngine> = Ratio<$n, A, B>;
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

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct RatioMode;

impl<const A: u8, B, C> Ratio<A, B, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn invert(self) -> Result<Self> {
        let ret: B = B::AS_1;
        let ret: Self = C::div(ret, self.n)?.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C> From<Unit<A, B, C>> for Ratio<A, B, C>
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
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}