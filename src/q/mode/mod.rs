use super::*;

::modwire::expose!(
    pub chance
    pub deg
    pub percentage
    pub rad
    pub ratio
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

pub trait Mode {}

ty!(
    1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

#[repr(transparent)]
pub struct DefaultMode;

impl Mode for DefaultMode {}

impl<const A: u8, B, C> From<B> for Q<A, B, DefaultMode, C> 
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    fn from(n: B) -> Self {
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Q<A, B, PercentageMode, C>> for Q<A, B, DefaultMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    fn from(q: Q<A, B, PercentageMode, C>) -> Self {
        let n: B = q.n;
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Q<A, B, RatioMode, C>> for Q<A, B, DefaultMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    fn from(q: Q<A, B, RatioMode, C>) -> Self {
        let n: B = q.n;
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Q<A, B, RadMode, C>> for Q<A, B, DefaultMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    fn from(q: Q<A, B, RadMode, C>) -> Self {
        let n: B = q.n;
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> From<Q<A, B, DegMode, C>> for Q<A, B, DefaultMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B>,
    (): ScaleCompatible<A, B>,
    (): PICompatible<A, B> {
    #[inline]
    fn from(q: Q<A, B, DegMode, C>) -> Self {
        let n: B = q.n;
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> ::core::ops::Add for Q<A, B, DefaultMode, C> 
where
    B: num::Int,
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

impl<const A: u8, B, C> ::core::ops::Sub for Q<A, B, DefaultMode, C> 
where
    B: num::Int,
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

impl<const A: u8, B, C> ::core::ops::Mul for Q<A, B, DefaultMode, C>
where
    B: num::Int,
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

impl<const A: u8, B, C> ::core::ops::Div for Q<A, B, DefaultMode, C>
where
    B: num::Int,
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