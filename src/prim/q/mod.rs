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

type Ratio<T> = T;
type Deg<T> = T;
type Rad<T> = T;

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
    pub fn tan(self) -> Result<Ratio<Self>> {
        let ret: B = self.n;
        let ret: B = C::tan(ret)?;
        let ret: Self = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<Self>> {
        let ret: B = self.n;
        let ret: B = C::sin(ret)?;
        let ret: Self = ret.into();
        Ok(ret)
    }

    /// Where `Self` is a radian
    #[inline]
    pub fn cos(self) -> Result<Ratio<Self>> {
        let ret: B = self.n;
        let ret: B = C::cos(ret)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
    
    // Should be degree
    #[inline]
    pub fn to_rad(self) -> Result<Deg<Self>> {
        let ret: B = self.n;
        let ret: B = C::to_rad(ret)?;
        let ret: Self = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn to_deg(self) -> Result<Rad<Self>> {
        let ret: B = self.n;
        let ret: B = C::to_deg(ret)?;
        let ret: Self = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn to_negative(self) -> Self {
        let ret: B = self.n;
        let ret: B = C::to_negative(ret);
        let ret: Self = ret.into();
        ret
    }

    #[inline]
    pub fn to_positive(self) -> Self {
        let ret: B = self.n;
        let ret: B = C::to_positive(ret);
        let ret: Self = ret.into();
        ret
    }

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