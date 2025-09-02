use super::*;

::modwire::expose!(
    pub r#as
    pub default_mode
    pub deg_mode
    pub engine
    pub mode
    pub pi
    pub rad_mode
    pub scale
    pub supported
);

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[cfg_attr(feature = "std", derive(::serde::Serialize))]
#[cfg_attr(feature = "std", derive(::serde::Deserialize))]
#[cfg_attr(feature = "std", derive(::thiserror::Error))]
pub enum Error {
    #[cfg_attr(feature = "std", error(""))]
    Overflow,
    #[cfg_attr(feature = "std", error(""))]
    Underflow,
    #[cfg_attr(feature = "std", error(""))]
    DivisionByZero,
    #[cfg_attr(feature = "std", error(""))]
    ModuloByZero,
    #[cfg_attr(feature = "std", error(""))]
    UnsupportedOperation
}

/// # Where
/// - `A`: Precision
#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
#[cfg_attr(feature = "std", derive(::serde::Serialize))]
#[cfg_attr(feature = "std", derive(::serde::Deserialize))]
pub struct Q<const A: u8, B = usize, C = DefaultMode, D = DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    n: B,
    mode: ::core::marker::PhantomData<C>,
    engine: ::core::marker::PhantomData<D>
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

    #[inline]
    pub fn as_int(self) -> B {
        self.n
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

impl<const A: u8, B, C, D> From<B> for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    // raw conversion no scaling involved.
    #[inline]
    fn from(n: B) -> Self {
        Self {
            n,
            mode: ::core::marker::PhantomData,
            engine: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C, D> ops::ToPrim for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn to_u8(&self) -> ops::Result<u8> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u8 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn to_u16(&self) -> ops::Result<u16> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u16 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn to_u32(&self) -> ops::Result<u32> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u32 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn to_u64(&self) -> ops::Result<u64> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u64 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn to_u128(&self) -> ops::Result<u128> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u128 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)   
    }

    fn to_usize(&self) -> ops::Result<usize> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: usize = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn to_i8(&self) -> ops::Result<i8> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i8 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn to_i16(&self) -> ops::Result<i16> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i16 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn to_i32(&self) -> ops::Result<i32> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i32 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)   
    }

    fn to_i64(&self) -> ops::Result<i64> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i64 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn to_i128(&self) -> ops::Result<i128> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i128 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    fn to_isize(&self) -> ops::Result<isize> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: isize = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)   
    }

    fn to_f32(&self) -> ops::Result<f32> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: f32 = ret.to_f32()?;
        Ok(ret)
    }

    fn to_f64(&self) -> ops::Result<f64> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: f64 = ret.to_f64()?;
        Ok(ret)
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
        let ret: B = D::div::<A, _>(x, y)?;
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

#[allow(clippy::non_canonical_partial_ord_impl)]
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
        let x: B = self.n;
        let y: B = other.n;
        let ret: ::core::cmp::Ordering = x.cmp(&y);
        Some(ret)
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
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "Q({})", self.n)
    }
}

impl<const A: u8, B, C, D> ::core::fmt::Display for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "Q({})", self.n)
    }
}

#[cfg(test)]
#[::rstest::rstest]
#[case(295_34, 295)]
fn test_int_conversion<T>(#[case] n: T, #[case] ok: u128)
where
    T: Into<Q2<u128>> {
    use ops::ToPrim as _;

    let n: Q2<u128> = n.into();
    let n: u128 = n.to_u128().unwrap();
    assert_eq!(n, ok);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(295_34, 295.0)]
fn test_float_conversion<T>(#[case] n: T, #[case] ok: f64) 
where
    T: Into<Q2<u128>> {
    use ops::ToPrim as _;

    let n: Q2<u128> = n.into();
    let n: f64 = n.to_f64().unwrap();
    assert_eq!(n, ok);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00.into(), 1_00.into(), 2_00.into())]
fn test_add(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
    let ret: Q2<_> = (x + y).unwrap();
    assert_eq!(ret, expected);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00.into(), 1_00.into(), 0_00.into())]
fn test_sub(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
    let ret: Q2<_> = (x - y).unwrap();
    assert_eq!(ret, expected);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00.into(), 1_00.into(), 1_00.into())]
#[case(0_50.into(), 0_25.into(), 0_12.into())]
fn test_mul(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
    let ret: Q2<_> = (x * y).unwrap();
    assert_eq!(ret, expected);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00.into(), 1_00.into(), 1_00.into())]
fn test_div(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
    let ret: Q2<_> = (x / y).unwrap();
    assert_eq!(ret, expected);
}