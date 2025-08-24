use super::*;

impl<const A: Precision, B, C, D> Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {

    #[inline]
    pub fn lerp<E, F>(self, rhs: E, t: F) -> Result<Self> 
    where
        E: Into<Self>,
        F: Into<Self> {
        let x: Self = self;
        let x: B = x.n;
        let y: Self = rhs.into();
        let y: B = y.n;
        let t: Self = t.into();
        let t: B = t.n;
        let ret: B = D::lerp(x, y, t)?;
        let ret: Self = ret.into();
        Ok(ret)
    }

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

impl<const A: Precision, B, C, D> Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn sqrt(self) -> Result<Self> {
        let ret: B = self.n;
        let ret: B = D::sqrt(ret)?;
        let ret: Self = ret.into();
        Ok(ret)
    }

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

impl<const A: Precision, B, C, D> From<B> for Q<A, B, C, D>
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
            mode: ::core::marker::PhantomData,
            engine: ::core::marker::PhantomData
        }
    }
}

impl<const A: Precision, B, C, D> ::core::ops::Add for Q<A, B, C, D>
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

impl<const A: Precision, B, C, D> ::core::ops::Sub for Q<A, B, C, D>
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

impl<const A: Precision, B, C, D> ::core::ops::Mul for Q<A, B, C, D>
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

impl<const A: Precision, B, C, D> ::core::ops::Div for Q<A, B, C, D>
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
        let ret: B = D::div::<A, _>(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: Precision, B, C, D> Eq for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {}

impl<const A: Precision, B, C, D> PartialEq for Q<A, B, C, D>
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

impl<const A: Precision, B, C, D> Ord for Q<A, B, C, D>
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

impl<const A: Precision, B, C, D> PartialOrd for Q<A, B, C, D>
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

#[cfg(test)]
#[allow(clippy::zero_prefixed_literal)]
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
}