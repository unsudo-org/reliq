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

type F<T> = T;
type Deg<T> = T;
type Rad<T> = T;
type Ratio<T> = T;

pub type Q1<T> = Q<1, T>;
pub type Q2<T> = Q<2, T>;
pub type Q3<T> = Q<3, T>;
pub type Q4<T> = Q<4, T>;
pub type Q5<T> = Q<5, T>;
pub type Q6<T> = Q<6, T>;
pub type Q7<T> = Q<7, T>;
pub type Q8<T> = Q<8, T>;
pub type Q9<T> = Q<9, T>;
pub type Q10<T> = Q<10, T>;
pub type Q11<T> = Q<11, T>;
pub type Q12<T> = Q<12, T>;
pub type Q13<T> = Q<13, T>;
pub type Q14<T> = Q<14, T>;
pub type Q15<T> = Q<15, T>;
pub type Q16<T> = Q<16, T>;
pub type Q17<T> = Q<17, T>;
pub type Q18<T> = Q<18, T>;
pub type Q19<T> = Q<19, T>;
pub type Q20<T> = Q<20, T>;
pub type Q21<T> = Q<21, T>;
pub type Q22<T> = Q<22, T>;
pub type Q23<T> = Q<23, T>;
pub type Q24<T> = Q<24, T>;
pub type Q25<T> = Q<25, T>;
pub type Q26<T> = Q<26, T>;
pub type Q27<T> = Q<27, T>;
pub type Q28<T> = Q<28, T>;
pub type Q29<T> = Q<29, T>;
pub type Q30<T> = Q<30, T>;
pub type Q31<T> = Q<31, T>;
pub type Q32<T> = Q<32, T>;
pub type Q33<T> = Q<33, T>;
pub type Q34<T> = Q<34, T>;
pub type Q35<T> = Q<35, T>;
pub type Q36<T> = Q<36, T>;
pub type Q37<T> = Q<37, T>;

pub type Deg1<T> = Q<1, T, DegMode>;
pub type Deg2<T> = Q<2, T, DegMode>;
pub type Deg3<T> = Q<3, T, DegMode>;
pub type Deg4<T> = Q<4, T, DegMode>;
pub type Deg5<T> = Q<5, T, DegMode>;
pub type Deg6<T> = Q<6, T, DegMode>;
pub type Deg7<T> = Q<7, T, DegMode>;
pub type Deg8<T> = Q<8, T, DegMode>;
pub type Deg9<T> = Q<9, T, DegMode>;
pub type Deg10<T> = Q<10, T, DegMode>;
pub type Deg11<T> = Q<11, T, DegMode>;
pub type Deg12<T> = Q<12, T, DegMode>;
pub type Deg13<T> = Q<13, T, DegMode>;
pub type Deg14<T> = Q<14, T, DegMode>;
pub type Deg15<T> = Q<15, T, DegMode>;
pub type Deg16<T> = Q<16, T, DegMode>;
pub type Deg17<T> = Q<17, T, DegMode>;
pub type Deg18<T> = Q<18, T, DegMode>;

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
    v: B,
    m_0: ::core::marker::PhantomData<C>,
    m_1: ::core::marker::PhantomData<D>
}

impl<const A: u8, B, C, D> Q<A, B, C, D> 
where
    B: num::Int,
    C: Mode,
    D: Engine,
    (): Precision<A>,
    (): N<B> {
    pub fn new(v: B) -> Self {
        Self {
            v,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C> ::core::ops::Add for Q<A, B, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Ok(Q::new(C::add(self.v, rhs.v)?))
    }
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
        Ok(Q::new(C::sub(self.v, rhs.v)?))
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
        Ok(Q::new(C::mul::<A, _>(self.v, rhs.v)?))
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
        Ok(Q::new(C::div::<A, _>(self.v, rhs.v)?))
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
        self.v == other.v
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
        let x: B = self.v;
        let y: B = other.v;
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


pub struct DefaultMode;

impl<const A: u8, B, C> Q<A, B, DefaultMode, C> 
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
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
        Q::new(value.v)
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
        Ok(Q::new(C::tan(self.v)?))
    }

    #[inline]
    pub fn sin(self) -> Result<Q<A, B, RatioMode, C>> {
        Ok(Q::new(C::sin(self.v)?))
    }

    #[inline]
    pub fn cos(self) -> Result<Q<A, B, RatioMode, C>> {
        Ok(Q::new(C::cos(self.v)?))
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
        Ok(Q::new(C::to_rad(value.v)?))
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
        Q::new(n.v)
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
        Ok(Q::new(C::to_deg(value.v)?))
    }
}


fn t() {
    let x: Q2<u128> = 50.into();
    let y: Deg2<u128> = 200.into();
}