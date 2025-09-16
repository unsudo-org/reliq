use super::*;

::modwire::expose!(
    pub r#as
    pub cardinal
    pub chance
    pub deg
    pub delta
    pub engine
    pub factor
    pub mode
    pub percentage
    pub pi
    pub rad
    pub ratio
    pub scale
    pub supported
    pub unit
);

pub type Precision = u8;

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(Hash)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIs)]
#[derive(::thiserror::Error)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub enum Error {
    #[error("{0}")]
    OpsError(#[from] ops::Error),
    #[error("Overflow.")]
    Overflow,
    #[error("Underflow.")]
    Underflow,
    #[error("Division by zero.")]
    DivisionByZero,
    #[error("Modulo by zero.")]
    ModuloByZero,
    #[error("Unsupported conversion.")]
    UnsupportedConversion,
    #[error("")]
    OutOfBounds
}

#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub struct Q<
    const A: Precision = 2, 
          B = usize, 
          C = UnitMode, 
          D = DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    n: B,
    m_0: ::core::marker::PhantomData<C>,
    m_1: ::core::marker::PhantomData<D>
}

impl<const A: Precision, B, C, D> Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
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
        let x: B = x.into_int();
        let y: Self = rhs.into();
        let y: B = y.into_int();
        let t: Self = t.into();
        let t: B = t.into_int();
        let ret: Self = D::lerp(x, y, t)?.into();
        Ok(ret)
    }

    #[inline]
    pub fn cast<const E: u8>(self) -> Result<lossy::Lossy<Q<E, B, C, D>>> 
    where
        (): SupportedPrecision<E>,
        (): Supported<E, B> {
        let ret: B = self.n;
        match D::cast::<A, E, B>(ret)? {
            lossy::Lossy::Exact(n) => Ok(lossy::Lossy::Exact(n.into())),
            lossy::Lossy::Trunc(n) => Ok(lossy::Lossy::Trunc(n.into()))
        }
    }

    pub fn round_up(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_up(n);
        let ret: Self = ret.into();
        ret
    }

    pub fn round_down(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_down(n);
        let ret: Self = ret.into();
        ret
    }

    pub fn round_towards_zero(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_towards_zero(n);
        let ret: Self = ret.into();
        ret
    }

    pub fn round_away_from_zero(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_away_from_zero(n);
        let ret: Self = ret.into();
        ret
    }

    #[inline]
    pub fn into_int(self) -> B {
        self.n
    }
}

impl<const A: u8, B, C, D> Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    B: ops::Signed,
    C: Mode,
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

impl<const A: u8, B, C, D, E> ::core::ops::Add<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Q<A, B, D, E>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = E::add(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C, D, E> ::core::ops::Sub<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Q<A, B, D, E>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = E::sub(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C, D, E> ::core::ops::Mul<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Q<A, B, D, E>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = E::mul::<A, _>(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C, D, E> ::core::ops::Div<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Q<A, B, D, E>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        let ret: B = E::div::<A, _>(x, y)?;
        let ret: Self = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C, D> Eq for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {}

impl<const A: u8, B, C, D, E> PartialEq<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {

    fn eq(&self, other: &Q<A, B, D, E>) -> bool {
        self == other
    }
}

impl<const A: u8, B, C, D> Ord for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {

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

impl<const A: u8, B, C, D, E> PartialOrd<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {

    fn ge(&self, other: &Q<A, B, D, E>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x >= y
    }

    fn le(&self, other: &Q<A, B, D, E>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x <= y
    }

    fn gt(&self, other: &Q<A, B, D, E>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x > y
    }

    fn lt(&self, other: &Q<A, B, D, E>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x < y
    }

    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Q<A, B, D, E>) -> Option<core::cmp::Ordering> {
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
    C: Mode,
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
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "Q({})", self.n)
    }
}

impl<const A: u8, B, C, D> ops::ToPrim for Q<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
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
    T: Into<Unit2<u128>> {
    use ops::ToPrim as _;

    

    let n: Q2<u128> = n.into();
    let n: f64 = n.to_f64().unwrap();
    assert_eq!(n, ok);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00.into(), 1_00.into(), 2_00.into())]
fn test_add(#[case] x: Unit2, #[case] y: Unit2, #[case] expected: Unit2) {
    let ret: Unit2 = (x + y).unwrap();
    ret.lerp(4_00, 5_00).unwrap();
    ret.lerp(5_00, 4).unwrap();
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