use super::*;

::modwire::expose!(
    pub engine
    pub mode
        pi
        scale
    pub supported
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

macro_rules! rad_ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Rad<const A: u8, B> = Q<A, B, RadMode>;
            $(
                pub type [< Rad $n >]<T> = Q<$n, T, RadMode>;
            )*
        );
    };
}

macro_rules! deg_ty {
    ($($n:literal)*) => {
        ::paste::paste!(
            pub type Deg<const A: u8, B> = Q<A, B, DegMode>;
            $(
                pub type [< Deg $n >]<T> = Q<$n, T, DegMode>;
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

rad_ty!(
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

deg_ty!(
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37
);

type Ratio<T> = T;

#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B = usize, C = DefaultMode, D = DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    n: B,
    m_0: ::core::marker::PhantomData<C>,
    m_1: ::core::marker::PhantomData<D>
}

impl<const A: u8, B, C, D> Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {

    #[inline]
    pub fn cast<const E: u8>(self) -> Result<Q<E, B, C, D>> 
    where
        (): SupportedPrecision<E>,
        (): Supported<E, B> {
        let ret: B = self.n;
        let ret: B = D::cast::<A, E, _>(ret)?;
        let ret: Q<E, B, C, D> = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C, D> Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn to_negative(self) -> Self {
        let ret: B = self.n;
        let ret: B = D::to_negative(ret);
        let ret: Self = ret.into();
        ret
    }

    #[inline]
    pub fn to_positive(self) -> Self {
        let ret: B = self.n;
        let ret: B = D::to_positive(ret);
        let ret: Self = ret.into();
        ret
    }
}

impl<const A: u8, B, C, D> From<B> for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn from(n: B) -> Self {
        Self {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C, D> ::core::fmt::Debug for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Q({})", self.n)
    }
}

impl<const A: u8, B, C, D> ::core::ops::Add for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = D::add(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C, D> ::core::ops::Sub for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = D::sub(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C, D> ::core::ops::Mul for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = D::mul(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C, D> ::core::ops::Div for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = D::div(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C, D> Eq for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {}

impl<const A: u8, B, C, D> PartialEq for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl<const A: u8, B, C, D> Ord for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
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

impl<const A: u8, B, C, D> PartialOrd for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
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


#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct DefaultMode;


#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct RadMode;

impl<const A: u8, B, C> TryFrom<Q<A, B, DegMode, C>> for Q<A, B, RadMode, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Q<A, B, DegMode, C>) -> ::core::result::Result<Self, Self::Error> {
        value.to_rad()
    }
}

impl<const A: u8, B, C> Q<A, B, RadMode, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        let ret: B = self.n;
        let ret: B = C::tan(ret)?;
        let ret: Q<A, B, DefaultMode, C> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        let ret: B = self.n;
        let ret: B = C::sin(ret)?;
        let ret: Q<A, B, DefaultMode, C> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        let ret: B = self.n;
        let ret: B = C::cos(ret)?;
        let ret: Q<A, B, DefaultMode, C> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn to_deg(self) -> Result<Q<A, B, DegMode, C>> {
        let ret: B = self.n;
        let ret: B = C::to_deg(ret)?;
        let ret: Q<A, B, DegMode, C> = ret.into();
        Ok(ret)
    }
}


#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct DegMode;

impl<const A: u8, B, C> TryFrom<Q<A, B, RadMode, C>> for Q<A, B, DegMode, C>
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Q<A, B, RadMode, C>) -> ::core::result::Result<Self, Self::Error> {
        value.to_deg()
    }
}

impl<const A: u8, B, C> Q<A, B, DegMode, C> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        self.to_rad()?.tan()
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        self.to_rad()?.sin()
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<Q<A, B, DefaultMode, C>>> {
        self.to_rad()?.cos()
    }

    #[inline]
    pub fn to_rad(self) -> Result<Q<A, B, RadMode, C>> {
        let ret: B = self.n;
        let ret: B = C::to_rad(ret)?;
        let ret: Q<A, B, RadMode, C> = ret.into();
        Ok(ret)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into(), 2_00.into())]
    fn add(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = (x + y).unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into(), 0_00.into())]
    fn sub(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = (x - y).unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into(), 1_00.into())]
    #[case(0_50.into(), 0_25.into(), 0_12.into())]
    fn mul(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = (x * y).unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into(), 1_00.into())]
    fn div(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = (x / y).unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(25_00.into(), 0_46.into())]
    fn tan(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = angle.tan().unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(25_00.into(), 0_42.into())]
    fn sin(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = angle.sin().unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into())]
    fn cos(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = angle.cos().unwrap();
        assert_eq!(ret, expected);
    }
}