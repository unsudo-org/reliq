use super::*;
use ops::ToPrim as _;

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
    pub util
);

#[macro_export(local_inner_macros)]
macro_rules! mode {
    (
        $(#[$attr:meta])*
        $mode:ident
    ) => {
        ::paste::paste!(
            $(#[$attr])*
            pub type $mode<const A: u8, B = usize, C = DefaultEngine> = Q<A, B, [< $mode Mode >], C>;
            
            $(#[$attr])*
            pub type [< $mode 1 >]<A = usize, B = DefaultEngine> = $mode<1, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 2 >]<A = usize, B = DefaultEngine> = $mode<2, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 3 >]<A = usize, B = DefaultEngine> = $mode<3, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 4 >]<A = usize, B = DefaultEngine> = $mode<4, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 5 >]<A = usize, B = DefaultEngine> = $mode<5, A, B>;
           
            $(#[$attr])*
            pub type [< $mode 6 >]<A = usize, B = DefaultEngine> = $mode<6, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 7 >]<A = usize, B = DefaultEngine> = $mode<7, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 8 >]<A = usize, B = DefaultEngine> = $mode<8, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 9 >]<A = usize, B = DefaultEngine> = $mode<9, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 10 >]<A = usize, B = DefaultEngine> = $mode<10, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 11 >]<A = usize, B = DefaultEngine> = $mode<11, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 12 >]<A = usize, B = DefaultEngine> = $mode<12, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 13 >]<A = usize, B = DefaultEngine> = $mode<13, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 14 >]<A = usize, B = DefaultEngine> = $mode<14, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 15 >]<A = usize, B = DefaultEngine> = $mode<15, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 16 >]<A = usize, B = DefaultEngine> = $mode<16, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 17 >]<A = usize, B = DefaultEngine> = $mode<17, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 18 >]<A = usize, B = DefaultEngine> = $mode<18, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 19 >]<A = usize, B = DefaultEngine> = $mode<19, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 20 >]<A = usize, B = DefaultEngine> = $mode<20, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 21 >]<A = usize, B = DefaultEngine> = $mode<21, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 22 >]<A = usize, B = DefaultEngine> = $mode<22, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 23 >]<A = usize, B = DefaultEngine> = $mode<23, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 24 >]<A = usize, B = DefaultEngine> = $mode<24, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 25 >]<A = usize, B = DefaultEngine> = $mode<25, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 26 >]<A = usize, B = DefaultEngine> = $mode<26, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 27 >]<A = usize, B = DefaultEngine> = $mode<27, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 28 >]<A = usize, B = DefaultEngine> = $mode<28, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 29 >]<A = usize, B = DefaultEngine> = $mode<29, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 30 >]<A = usize, B = DefaultEngine> = $mode<30, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 31 >]<A = usize, B = DefaultEngine> = $mode<31, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 32 >]<A = usize, B = DefaultEngine> = $mode<32, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 33 >]<A = usize, B = DefaultEngine> = $mode<33, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 34 >]<A = usize, B = DefaultEngine> = $mode<34, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 35 >]<A = usize, B = DefaultEngine> = $mode<35, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 36 >]<A = usize, B = DefaultEngine> = $mode<36, A, B>;
            
            $(#[$attr])*
            pub type [< $mode 37 >]<A = usize, B = DefaultEngine> = $mode<37, A, B>;

            #[derive(Debug)]
            #[derive(Clone)]
            #[derive(Copy)]
            pub struct [< $mode Mode >];

            impl Mode for [< $mode Mode >] {}
        );
    };
}

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
    const PRECISION: Precision = 2,
          B = usize, 
          C = UnitMode, 
          D = DefaultEngine>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<PRECISION>,
    (): SupportedInt<B>,
    (): Supported<PRECISION, B> {
    n: B,
    m_0: ::core::marker::PhantomData<C>,
    m_1: ::core::marker::PhantomData<D>
}

impl<const A: Precision, B, C, D> Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {

    #[inline]
    pub fn abs(self) -> Self {
        let ret: Self = D::abs(self.n).into();
        ret
    }

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
    pub fn cast<const E: u8>(self) -> Result<Q<E, B, C, D>> 
    where
        (): SupportedPrecision<E>,
        (): Supported<E, B> {
        let ret: B = self.n;
        let ret: B = D::cast::<A, E, B>(ret)?;
        let ret: Q<E, B, C, D> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn round_up(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_up(n);
        let ret: Self = ret.into();
        ret
    }

    #[inline]
    pub fn round_down(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_down(n);
        let ret: Self = ret.into();
        ret
    }

    #[inline]
    pub fn round_towards_zero(self) -> Self {
        let n: B = self.n;
        let ret: B = D::round_towards_zero(n);
        let ret: Self = ret.into();
        ret
    }

    #[inline]
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

impl<const A: u8, B, C, D> From<B> for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn from(value: B) -> Self {
        Self {
            n: value,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}

impl<const A: u8, B, C, D, E> ::core::ops::Add<Result<Q<A, B, D, E>>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;

    #[inline]
    fn add(self, rhs: Result<Q<A, B, D, E>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self + rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Add<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;

    #[inline]
    fn add(self, rhs: Q<A, B, D, E>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        match E::add(x, y) {
            Ok(n) => {
                let n: Self = n.into();
                Ok(n)
            },
            Err(e) => Err::<Q<A, B, C, E>, Error>(e).into()
        }
    }
}

impl<const A: u8, B, C, D> ::core::ops::Add<B> for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, D>>;
    
    #[inline]
    fn add(self, rhs: B) -> Self::Output {
        let rhs: Self = rhs.into();
        self + rhs
    }
}

impl<const A: u8, B, C, D, E> ::core::ops::Sub<Result<Q<A, B, D, E>>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;

    fn sub(self, rhs: Result<Q<A, B, D, E>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self - rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B, C, D, E> ::core::ops::Sub<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;

    #[inline]
    fn sub(self, rhs: Q<A, B, D, E>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        match E::sub(x, y) {
            Ok(n) => {
                let n: Self = n.into();
                Ok(n)
            },
            Err(e) => Err::<Q<A, B, C, E>, Error>(e).into()
        }
    }
}

impl<const A: u8, B, C, D> ::core::ops::Sub<B> for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, D>>;

    #[inline]
    fn sub(self, rhs: B) -> Self::Output {
        let rhs: Self = rhs.into();
        self - rhs
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Mul<Result<Q<A, B, D, E>>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;
    
    #[inline]
    fn mul(self, rhs: Result<Q<A, B, D, E>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self * rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Mul<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;

    #[inline]
    fn mul(self, rhs: Q<A, B, D, E>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        match E::mul::<A, B>(x, y) {
            Ok(n) => {
                let n: Self = n.into();
                Ok(n)
            },
            Err(e) => Err::<Q<A, B, C, E>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D> ::core::ops::Mul<B> for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, D>>;

    #[inline]
    fn mul(self, rhs: B) -> Self::Output {
        let rhs: Self = rhs.into();
        self * rhs
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Div<Result<Q<A, B, D, E>>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;
    
    #[inline]
    fn div(self, rhs: Result<Q<A, B, D, E>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self * rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Div<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;

    #[inline]
    fn div(self, rhs: Q<A, B, D, E>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        match E::div::<A, B>(x, y) {
            Ok(n) => {
                let n: Self = n.into();
                Ok(n)
            },
            Err(e) => Err::<Q<A, B, C, E>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D> ::core::ops::Div<B> for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, D>>;

    #[inline]
    fn div(self, rhs: B) -> Self::Output {
        let rhs: Self = rhs.into();
        self / rhs
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Rem<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;

    #[inline]
    fn rem(self, rhs: Q<A, B, D, E>) -> Self::Output {
        let x: B = self.n;
        let y: B = rhs.n;
        match E::rem::<A, B>(x, y) {
            Ok(n) => {
                let n: Self = n.into();
                Ok(n)
            },
            Err(e) => Err::<Q<A, B, C, E>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Rem<Result<Q<A, B, D, E>>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, E>>;

    #[inline]
    fn rem(self, rhs: Result<Q<A, B, D, E>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self % rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: Precision, B, C, D> ::core::ops::Rem<B> for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Result<Q<A, B, C, D>>;

    #[inline]
    fn rem(self, rhs: B) -> Self::Output {
        let rhs: Self = rhs.into();
        self % rhs
    }
}

impl<const A: u8, B, C, D> Eq for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {}

impl<const A: u8, B, C, D, E> PartialEq<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn eq(&self, other: &Q<A, B, D, E>) -> bool {
        self == other
    }
}

impl<const A: u8, B, C, D> PartialEq<B> for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn eq(&self, other: &B) -> bool {
        &self.n == other
    }
}

impl<const A: u8, B, C, D> PartialEq<f32> for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        &self.to_f32().unwrap_or_default() == other
    }
}

impl<const A: u8, B, C, D> PartialEq<f64> for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        &self.n.to_f64().unwrap_or_default() == other
    }
}

impl<const A: u8, B, C, D> Ord for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
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

impl<const A: u8, B, C, D, E> PartialOrd<Q<A, B, D, E>> for Q<A, B, C, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    
    #[inline]
    fn ge(&self, other: &Q<A, B, D, E>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x >= y
    }

    #[inline]
    fn le(&self, other: &Q<A, B, D, E>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x <= y
    }

    #[inline]
    fn gt(&self, other: &Q<A, B, D, E>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x > y
    }

    #[inline]
    fn lt(&self, other: &Q<A, B, D, E>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x < y
    }
    
    #[allow(clippy::non_canonical_partial_ord_impl)]
    #[inline]
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
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "Q({})", self.n)
    }
}

impl<const A: u8, B, C, D> ::core::fmt::Display for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "Q({})", self.n)
    }
}

impl<const A: u8, B, C, D> ops::ToPrim for Q<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn to_u8(&self) -> ops::Result<u8> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u8 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_u16(&self) -> ops::Result<u16> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u16 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_u32(&self) -> ops::Result<u32> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u32 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_u64(&self) -> ops::Result<u64> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u64 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_u128(&self) -> ops::Result<u128> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u128 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)   
    }

    #[inline]
    fn to_usize(&self) -> ops::Result<usize> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: usize = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_i8(&self) -> ops::Result<i8> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i8 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_i16(&self) -> ops::Result<i16> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i16 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_i32(&self) -> ops::Result<i32> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i32 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)   
    }

    #[inline]
    fn to_i64(&self) -> ops::Result<i64> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i64 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_i128(&self) -> ops::Result<i128> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i128 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_isize(&self) -> ops::Result<isize> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: isize = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)   
    }

    #[inline]
    fn to_f32(&self) -> ops::Result<f32> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: f32 = ret.to_f32()?;
        Ok(ret)
    }

    #[inline]
    fn to_f64(&self) -> ops::Result<f64> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: f64 = ret.to_f64()?;
        Ok(ret)
    }
}

#[cfg(test)]
#[::rstest::rstest]
#[case(295_34, 295_00)]
fn test_int_conversion<A, B>(#[case] n: A, #[case] ok: B)
where
    A: Into<Unit2>,
    B: Into<Unit2> {
    use ops::ToPrim as _;
    let ok: Unit2 = ok.into();
    let n: Unit2 = n.into();
    let n: usize = n.to_usize().unwrap();
    let n: Unit2 = n.into();
    assert_eq!(ok, n);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(295_34, 295_00)]
fn test_float_conversion<A, B>(#[case] n: A, #[case] ok: B) 
where
    A: Into<Unit2>,
    B: Into<Unit2> {
    use ops::ToPrim as _;
    let ok: Unit2 = ok.into();
    let n: Unit2 = n.into();
    let n: f64 = n.to_f64().unwrap();
    assert_eq!(ok, n);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00, 1_00, 2_00)]
fn test_add<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C) 
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

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00, 1_00, 0_00)]
#[case(2_00, 1_00, 1_00)]
fn test_sub<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C) 
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

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00, 1_00, 1_00)]
#[case(0_50, 0_25, 0_12)]
fn test_mul<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C) 
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

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00, 1_00, 1_00)]
#[case(1_00, 0_50, 2_00)]
fn test_div<A, B, C>(#[case] x: A, #[case] y: B, #[case] ok: C) 
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