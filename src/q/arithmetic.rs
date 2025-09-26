use super::*;

impl<const A: u8, B, C> Q<A, B, C> 
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn abs(self) -> Self {
        let n: B = self.n;
        if n < B::AS_0 {
            let n: B = B::AS_0 - n;
            let ret: Self = Self {
                n,
                m_0: ::core::marker::PhantomData
            };
            return ret
        }
        self
    }
}

impl<const A: u8, B, C> ::core::ops::Add<B> for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;
    
    #[inline]
    fn add(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = Self {
            n,
            m_0: ::core::marker::PhantomData
        };
        self + rhs
    }
}

impl<const A: u8, B, C, D> ::core::ops::Add<Result<Q<A, B, C>>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Result<Q<A, B, C>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self + rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B, C, D> ::core::ops::Add<Q<A, B, C>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Q<A, B, C>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        add(x, y).map(|n| {
            Self {
                n,
                m_0: ::core::marker::PhantomData
            }
        })
    }
}

impl<const A: u8, B, C> ::core::ops::Sub<B> for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;
    
    #[inline]
    fn sub(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = Self {
            n,
            m_0: ::core::marker::PhantomData
        };
        self - rhs
    }
}

impl<const A: u8, B, C, D> ::core::ops::Sub<Result<Q<A, B, C>>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Result<Q<A, B, C>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self - rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B, C, D> ::core::ops::Sub<Q<A, B, C>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Q<A, B, C>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        sub(x, y).map(|n| {
            Self {
                n,
                m_0: ::core::marker::PhantomData
            }
        })
    }
}

impl<const A: u8, B, C> ::core::ops::Mul<B> for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;
    
    #[inline]
    fn mul(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = Self {
            n,
            m_0: ::core::marker::PhantomData
        };
        self * rhs
    }
}

impl<const A: u8, B, C, D> ::core::ops::Mul<Result<Q<A, B, C>>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Result<Q<A, B, C>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self * rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B, C, D> ::core::ops::Mul<Q<A, B, C>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Q<A, B, C>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        mul(x, y).map(|n| {
            Self {
                n,
                m_0: ::core::marker::PhantomData
            }
        })
    }
}

impl<const A: u8, B, C> ::core::ops::Div<B> for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;
    
    #[inline]
    fn div(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = Self {
            n,
            m_0: ::core::marker::PhantomData
        };
        self / rhs
    }
}

impl<const A: u8, B, C, D> ::core::ops::Div<Result<Q<A, B, C>>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Result<Q<A, B, C>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self / rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B, C, D> ::core::ops::Div<Q<A, B, C>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Q<A, B, C>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        div(x, y).map(|n| {
            Self {
                n,
                m_0: ::core::marker::PhantomData
            }
        })
    }
}

impl<const A: u8, B, C> ::core::ops::Rem<B> for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;
    
    #[inline]
    fn rem(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = Self {
            n,
            m_0: ::core::marker::PhantomData
        };
        self % rhs
    }
}

impl<const A: u8, B, C, D> ::core::ops::Rem<Result<Q<A, B, C>>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn rem(self, rhs: Result<Q<A, B, C>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self % rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B, C, D> ::core::ops::Rem<Q<A, B, C>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn rem(self, rhs: Q<A, B, C>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        rem(x, y).map(|n| {
            Self {
                n,
                m_0: ::core::marker::PhantomData
            }
        })
    }
}

#[inline]
pub(super) fn add<T>(x: T, y: T) -> Result<T>
where
    T: ops::Int,
    (): SupportedInt<T> {
    x.checked_add(y).ok_or(Error::Overflow)
}

#[inline]
pub(super) fn sub<T>(x: T, y: T) -> Result<T>
where
    T: ops::Int,
    (): SupportedInt<T> {
    x.checked_sub(y).ok_or(Error::Underflow)
}

#[inline]
pub(super) fn mul<const A: u8, B>(x: B, y: B) -> Result<B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    if A == 0 {
        return x.checked_mul(y).ok_or(Error::Overflow)
    }
    muldiv(x, y, scale::<A, _>())
}

#[inline]
pub(super) fn div<const A: u8, B>(x: B, y: B) -> Result<B> 
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    if A == 0 {
        return x.checked_div(y).ok_or(Error::DivisionByZero)
    }
    muldiv(x, scale::<A, _>(), y)
}

#[inline]
pub(super) fn rem<const A: u8, B>(x: B, y: B) -> Result<B> 
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    if y == B::AS_0 {
        return Err(Error::DivisionByZero)
    }
    let n: B = div::<A, _>(x, y)?;
    let n: B = mul::<A, _>(n, y)?;
    let n: B = sub(x, n)?;
    Ok(n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[::rstest::rstest]
    #[case(1_00, 1_00, 2_00)]
    fn add<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C) 
    where
        A: Into<Unit2>,
        B: Into<Unit2>,
        C: Into<Unit2> {
        let x: Unit2 = x.into();
        let y: Unit2 = y.into();
        let ok: Unit2 = ok.into();
        let ret: Unit2 = (x + y).unwrap();
        assert_eq!(ret, ok);
    }

    #[::rstest::rstest]
    #[case(1_00, 1_00, 0_00)]
    #[case(2_00, 1_00, 1_00)]
    fn sub<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C) 
    where
        A: Into<Unit2>,
        B: Into<Unit2>,
        C: Into<Unit2> {
        let x: Unit2 = x.into();
        let y: Unit2 = y.into();
        let ok: Unit2 = ok.into();
        let ret: Unit2 = (x - y).unwrap();
        assert_eq!(ret, ok);
    }

    #[::rstest::rstest]
    #[case(1_00, 1_00, 1_00)]
    #[case(0_50, 0_25, 0_12)]
    fn mul<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C) 
    where
        A: Into<Unit2>,
        B: Into<Unit2>,
        C: Into<Unit2> {
        let x: Unit2 = x.into();
        let y: Unit2 = y.into();
        let ok: Unit2 = ok.into();
        let ret: Unit2 = (x * y).unwrap();
        assert_eq!(ret, ok);
    }

    #[::rstest::rstest]
    #[case(1_00, 1_00, 1_00)]
    #[case(1_00, 0_50, 2_00)]
    fn div<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C) 
    where
        A: Into<Unit2>,
        B: Into<Unit2>,
        C: Into<Unit2> {
        let x: Unit2 = x.into();
        let y: Unit2 = y.into();
        let ok: Unit2 = ok.into();
        let ret: Unit2 = (x / y).unwrap();
        assert_eq!(ret, ok);
    }

    #[::rstest::rstest]
    #[case(1_00, 1_00, 1_00)]
    fn rem<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C)
    where
        A: Into<Unit2>,
        B: Into<Unit2>,
        C: Into<Unit2> {
        let x: Unit2 = x.into();
        let y: Unit2 = y.into();
        let ok: Unit2 = ok.into();
        let ret: Unit2 = (x % y).unwrap();
        assert_eq!(ret, ok);
    }
}