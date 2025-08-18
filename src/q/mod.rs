use super::*;

::modwire::expose!(
    pub engine
        fold
    pub mode
        n
        pi
    pub precision
        scale
        trig
        wide_mul
);

macro_rules! ty_q {
    ($($n:literal)*) => {
        $(
            ::paste::paste!(
                pub type [< Q $n >]<T> = Q<$n, T>;
            );
        )*
    };
}

macro_rules! ty_deg {
    ($($n:literal)*) => {
        pub type Deg<const A: u8, B> = Q<A, B, DegMode>;
        $(
            ::paste::paste!(
                pub type [< Deg $n >]<T> = Q<$n, T, DegMode>;
            );
        )*
    };
}

macro_rules! ty_rad {
    ($($n:literal)*) => {
        pub type Rad<const A: u8, B> = Q<A, B, RadMode>;
        $(
            ::paste::paste!(
                pub type [< Rad $n >]<T> = Q<$n, T, RadMode>;
            );
        )*
    };
}

ty_q!(
    1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

ty_deg!(
    1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

ty_rad!(
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

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B = usize, C = DefaultMode, D = DefaultEngine>
where
    B: num::Int,
    C: Mode,
    D: Engine,
    (): Precision<A>,
    (): N<B> {
    n: B,
    m_0: ::core::marker::PhantomData<C>,
    m_1: ::core::marker::PhantomData<D>
}


impl<const A: u8, B, C> ::core::ops::Sub for Q<A, B, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Ok(Q::new(C::sub(self.n, rhs.n)?))
    }
}

impl<const A: u8, B, C> ::core::ops::Mul for Q<A, B, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Ok(Q::new(C::mul::<A, _>(self.n, rhs.n)?))
    }
}

impl<const A: u8, B, C> ::core::ops::Div for Q<A, B, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Ok(Q::new(C::div::<A, _>(self.n, rhs.n)?))
    }
}

impl<const A: u8, B, C> Eq for Q<A, B, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> 
{}

impl<const A: u8, B, C> PartialEq for Q<A, B, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl<const A: u8, B, C> Ord for Q<A, B, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
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
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}









#[repr(transparent)]
pub struct DefaultMode;

impl<const A: u8, B, C> From<B> for Q<A, B, DefaultMode, C> 
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    fn from(n: B) -> Self {
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
    (): N<B> {
    fn from(n: Q<A, B, RadMode, C>) -> Self {
        n.n.into()
    }
}

impl<const A: u8, B, C> From<Q<A, B, DegMode, C>> for Q<A, B, DefaultMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    fn from(n: Q<A, B, DegMode, C>) -> Self {
        n.n.into()
    }
}

impl<const A: u8, B, C> ::core::ops::Add for Q<A, B, DefaultMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Output = Result<Self>;

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
    (): N<B> {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = C::sub(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}










// ===

pub struct RatioMode;

impl<const A: u8, B, C> Q<A, B, RatioMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    
}

impl<const A: u8, B, C> From<Q<A, B, DefaultMode, C>> for Q<A, B, RatioMode, C> 
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    fn from(value: Q<A, B, DefaultMode, C>) -> Self {
        Q::new(value.n)
    }
}


// ===

pub struct PercentageMode;

impl<const A: u8, B, C> Q<A, B, PercentageMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    pub fn of(self, n: Q<A, B, DefaultMode, C>) -> Result<Q<A, B, DefaultMode, C>> {

    }
}


// ===

pub struct ChanceMode;

impl<const A: u8, B, C> Q<A, B, ChanceMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    pub fn roll<const D: usize>(self, seed: [u8; D]) -> bool {

    }
}

impl<const A: u8, B, C> TryFrom<B> for Q<A, B, ChanceMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Error = Error;
    
    fn try_from(n: B) -> ::core::result::Result<Self, Self::Error> {
        if n < B::AS_0 {
            return Err()
        }
        if n > B::AS_1 {
            return Err()
        }
        Ok(Q::new(n))
    }
}


// ===

pub struct RadMode;

impl<const A: u8, B, C> Q<A, B, RadMode, C> 
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    pub fn tan(self) -> Result<Q<A, B, RatioMode, C>> {
        Ok(Q::new(C::tan(self.n)?))
    }

    #[inline]
    pub fn sin(self) -> Result<Q<A, B, RatioMode, C>> {
        Ok(Q::new(C::sin(self.n)?))
    }

    #[inline]
    pub fn cos(self) -> Result<Q<A, B, RatioMode, C>> {
        Ok(Q::new(C::cos(self.n)?))
    }
}

impl<const A: u8, B, C> TryFrom<Q<A, B, DegMode, C>> for Q<A, B, RadMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Error = Error;

    fn try_from(value: Q<A, B, DegMode, C>) -> ::core::result::Result<Self, Self::Error> {
        Ok(Q::new(C::to_rad(value.n)?))
    }
}


// ===

pub struct DegMode;

impl<const A: u8, B, C> From<B> for Q<A, B, DegMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    fn from(n: B) -> Self {
        Q::new(n)
    }
}

impl<const A: u8, B, C> From<Q<A, B, DefaultMode, C>> for Q<A, B, DegMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    fn from(n: Q<A, B, DefaultMode, C>) -> Self {
        Q::new(n.n)
    }
}

impl<const A: u8, B, C> TryFrom<Q<A, B, RadMode, C>> for Q<A, B, DegMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Error = Error;

    fn try_from(value: Q<A, B, RadMode, C>) -> ::core::result::Result<Self, Self::Error> {
        Ok(Q::new(C::to_deg(value.n)?))
    }
}


fn t() {
    let x: Q2<u128> = 50.into();
    let y: Deg2<u128> = x.into();
    let y: Rad2<u128> = y.try_into().ok().unwrap();
    y.cos();
}