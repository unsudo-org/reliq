use super::*;

pub enum Fragment<const A: Precision, B, C, D> 
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    Ok(Q<A, B, C, D>),
    Err(Error)
}

impl<const A: Precision, B, C, D> Fragment<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    pub fn into_result(self) -> Result<Q<A, B, C, D>> {
        match self {
            Self::Ok(x) => Ok(x),
            Self::Err(e) => Err(e)
        }
    }

    pub fn ok(self) -> Option<Q<A, B, C, D>> {
        match self {
            Self::Ok(x) => Some(x),
            Self::Err(_) => None
        }
    }
}

impl<const A: Precision, B, C, D> From<Result<Q<A, B, C, D>>> for Fragment<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Result<Q<A, B, C, D>>) -> Self {
        let outcome: Result<Q<A, B, C, D>> = value;
        match outcome {
            Ok(n) => Self::Ok(n),
            Err(e) => Self::Err(e)
        }
    }
}

impl<const A: Precision, B, C, D> From<Q<A, B, C, D>> for Fragment<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Q<A, B, C, D>) -> Self {
        let n: Q<A, B, C, D> = value;
        Self::Ok(n)
    }
}

impl<const A: Precision, B, C, D> From<B> for Fragment<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: B) -> Self {
        let n: Q<A, B, C, D> = value.into();
        Self::Ok(n)
    }
}

impl<const A: Precision, B, C, D> From<Result<B>> for Fragment<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Result<B>) -> Self {
        match value {
            Ok(n) => n.into(),
            Err(e) => Err::<Q<A, B, C, D>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Add<Fragment<A, B, C, E>> for Fragment<A, B, D, E>
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

    fn add(self, rhs: Fragment<A, B, C, E>) -> Self::Output {
        match (self, rhs) {
            (Self::Ok(lhs), Fragment::Ok(rhs)) => lhs + rhs,
            (Self::Ok(_), Fragment::Err(e)) 
            | (Self::Err(e), Fragment::Ok(_)) 
            | (Self::Err(e), Fragment::Err(_))=> Err::<Q<A, B, D, E>, Error>(e).into(),
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Add<Q<A, B, C, E>> for Fragment<A, B, D, E>
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

    fn add(self, rhs: Q<A, B, C, E>) -> Self::Output {
        match self {
            Self::Ok(lhs) => match lhs + rhs {
                Self::Ok(x) => x.into(),
                Self::Err(e) => Err(e).into()
            },
            Self::Err(e) => Err(e).into()
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

    fn sub(self, rhs: Fragment<A, B, C, E>) -> Self::Output {
        match (self, rhs) {
            (Self::Ok(lhs), Fragment::Ok(rhs)) => lhs - rhs,
            (Self::Ok(_), Fragment::Err(e)) 
            | (Self::Err(e), Fragment::Ok(_)) 
            | (Self::Err(e), Fragment::Err(_))=> Err(e).into(),
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
    type Output = ::core::ops::ControlFlow<Fragment<A, B, D, E>, Fragment<A, B, D, E>>;
    
    #[inline]
    fn sub(self, rhs: Q<A, B, C, E>) -> Self::Output {
        match self {
            Self::Ok(lhs) => match lhs + rhs {
                Self::Ok(x) => x.into(),
                Self::Err(e) => Err(e).into()
            },
            Self::Err(e) => Err(e).into()
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
            (Self::Ok(lhs), Fragment::Ok(rhs)) => lhs == rhs,
            (Self::Err(lhs), Fragment::Err(rhs)) => lhs == rhs,
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
        if let Self::Ok(x) = self {
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
        if let Self::Ok(x) = self {
            &x.into_int() == other
        } else {
            false
        }
    }
}