use super::*;

pub enum Fragment<const A: Precision, B, C, D> 
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    Success(Q<A, B, C, D>),
    Failure(Error)
}

impl<const A: Precision, B, C, D> Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    pub fn unwrap(self) -> Q<A, B, C, D> {
        if let Self::Success(n) = self {
            return n
        }
        panic!()
    }

    pub fn into_result(self) -> Result<Q<A, B, C, D>> {
        match self {
            Self::Success(x) => Ok(x),
            Self::Failure(e) => Err(e)
        }
    }

    pub fn ok(self) -> Option<Q<A, B, C, D>> {
        match self {
            Self::Success(x) => Some(x),
            Self::Failure(_) => None
        }
    }
}

impl<const A: Precision, B, C, D> From<Result<Q<A, B, C, D>>> for Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn from(value: Result<Q<A, B, C, D>>) -> Self {
        let outcome: Result<Q<A, B, C, D>> = value;
        match outcome {
            Ok(n) => Self::Success(n),
            Err(e) => Self::Failure(e)
        }
    }
}

impl<const A: Precision, B, C, D> From<Error> for Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn from(value: Error) -> Self {
        Self::Failure(value)
    }
}

impl<const A: Precision, B, C, D> From<Q<A, B, C, D>> for Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn from(value: Q<A, B, C, D>) -> Self {
        let n: Q<A, B, C, D> = value;
        Self::Success(n)
    }
}

impl<const A: Precision, B, C, D> From<Result<B>> for Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn from(value: Result<B>) -> Self {
        match value {
            Ok(n) => n.into(),
            Err(e) => Err::<Q<A, B, C, D>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D> From<B> for Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn from(value: B) -> Self {
        let n: Q<A, B, C, D> = value.into();
        Self::Success(n)
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Add<Fragment<A, B, C, E>> for Fragment<A, B, D, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, D, E>;
    
    #[inline]
    fn add(self, rhs: Fragment<A, B, C, E>) -> Self::Output {
        match (self, rhs) {
            (Self::Success(lhs), Fragment::Success(rhs)) => lhs + rhs,
            (Self::Success(_), Fragment::Failure(e)) 
            | (Self::Failure(e), Fragment::Success(_)) 
            | (Self::Failure(e), Fragment::Failure(_))=> Err::<Q<A, B, D, E>, Error>(e).into(),
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Add<Q<A, B, C, E>> for Fragment<A, B, D, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, D, E>;
    
    #[inline]
    fn add(self, rhs: Q<A, B, C, E>) -> Self::Output {
        match self {
            Self::Success(lhs) => match lhs + rhs {
                Self::Success(n) => n.into(),
                Self::Failure(e) => Err::<Q<A, B, D, E>, Error>(e).into()
            },
            Self::Failure(e) => Err::<Q<A, B, D, E>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D> ::core::ops::Add<B> for Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, C, D>;

    #[inline]
    fn add(self, rhs: B) -> Self::Output {
        match self {
            Self::Success(lhs) => lhs + rhs,
            Self::Failure(e) => Err::<Q<A, B, C, D>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Sub<Fragment<A, B, C, E>> for Fragment<A, B, D, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, D, E>;
    
    #[inline]
    fn sub(self, rhs: Fragment<A, B, C, E>) -> Self::Output {
        match (self, rhs) {
            (Self::Success(lhs), Fragment::Success(rhs)) => lhs - rhs,
            (Self::Success(_), Fragment::Failure(e)) 
            | (Self::Failure(e), Fragment::Success(_)) 
            | (Self::Failure(e), Fragment::Failure(_)) => Err::<Q<A, B, D, E>, Error>(e).into(),
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Sub<Q<A, B, C, E>> for Fragment<A, B, D, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, D, E>;
    
    #[inline]
    fn sub(self, rhs: Q<A, B, C, E>) -> Self::Output {
        match self {
            Self::Success(lhs) => match lhs + rhs {
                Self::Success(n) => n.into(),
                Self::Failure(e) => Err::<Q<A, B, D, E>, Error>(e).into()
            },
            Self::Failure(e) => Err::<Q<A, B, D, E>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D> ::core::ops::Sub<B> for Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, C, D>;
    
    #[inline]
    fn sub(self, rhs: B) -> Self::Output {
        match self {
            Self::Success(lhs) => lhs - rhs,
            Self::Failure(e) => Err::<Q<A, B, C, D>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Mul<Fragment<A, B, C, E>> for Fragment<A, B, D, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, D, E>;
    
    fn mul(self, rhs: Fragment<A, B, C, E>) -> Self::Output {
        match (self, rhs) {
            (Self::Success(lhs), Fragment::Success(rhs)) => lhs * rhs,
            (Self::Success(_), Fragment::Failure(e)) 
            | (Self::Failure(e), Fragment::Success(_)) 
            | (Self::Failure(e), Fragment::Failure(_)) => Err::<Q<A, B, D, E>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Mul<Q<A, B, C, E>> for Fragment<A, B, D, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, D, E>;

    #[inline]
    fn mul(self, rhs: Q<A, B, C, E>) -> Self::Output {
        match self {
            Self::Success(lhs) => match lhs * rhs {
                Self::Success(n) => n.into(),
                Self::Failure(e) => Err::<Q<A, B, D, E>, Error>(e).into()
            },
            Self::Failure(e) => Err::<Q<A, B, D, E>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D> ::core::ops::Mul<B> for Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, C, D>;
    
    #[inline]
    fn mul(self, rhs: B) -> Self::Output {
        match self {
            Self::Success(lhs) => lhs * rhs,
            Self::Failure(e) => Err::<Q<A, B, C, D>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Div<Fragment<A, B, C, E>> for Fragment<A, B, D, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Self;

    fn div(self, rhs: Fragment<A, B, C, E>) -> Self::Output {
        match (self, rhs) {
            (Self::Success(lhs), Fragment::Success(rhs)) => lhs / rhs,
            (Self::Success(_), Fragment::Failure(e))
            | (Self::Failure(e), Fragment::Success(_))
            | (Self::Failure(e), Fragment::Failure(_)) => Err::<Q<A, B, D, E>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Div<Q<A, B, C, E>> for Fragment<A, B, D, E>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, D, E>;

    #[inline]
    fn div(self, rhs: Q<A, B, C, E>) -> Self::Output {
        match self {
            Self::Success(lhs) => match lhs / rhs {
                Self::Success(n) => n.into(),
                Self::Failure(e) => Err::<Q<A, B, D, E>, Error>(e).into()
            },
            Self::Failure(e) => Err::<Q<A, B, D, E>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D> ::core::ops::Div<B> for Fragment<A, B, C, D>
where
    B: ops::Int,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = Fragment<A, B, C, D>;
    
    #[inline]
    fn div(self, rhs: B) -> Self::Output {
        match self {
            Self::Success(lhs) => lhs / rhs,
            Self::Failure(e) => Err::<Q<A, B, C, D>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D, E> PartialEq<Fragment<A, B, D, E>> for Fragment<A, B, C, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn eq(&self, other: &Fragment<A, B, D, E>) -> bool {
        match (self, other) {
            (Self::Success(lhs), Fragment::Success(rhs)) => lhs == rhs,
            (Self::Failure(lhs), Fragment::Failure(rhs)) => lhs == rhs,
            _ => false
        }
    }
}

impl<const A: Precision, B, C, D, E> PartialEq<Q<A, B, D, E>> for Fragment<A, B, C, E>
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
        if let Self::Success(x) = self {
            x == other
        } else {
            false
        }
    }
}

impl<const A: Precision, B, C, D> PartialEq<B> for Fragment<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn eq(&self, other: &B) -> bool {
        if let Self::Success(x) = self {
            &x.into_int() == other
        } else {
            false
        }
    }
}