use super::*;

pub enum Partial<T> {
    Ok(T),
    Err(Error)
}

impl<const A: Precision, B, C, D> From<Result<Q<A, B, C, D>>> for Partial<Q<A, B, C, D>>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Result<Q<A, B, C, D>>) -> Self {
        match value {
            Ok(n) => Self::Ok(n),
            Err(e) => Self::Err(e)
        }
    }
}

impl<const A: Precision, B, C, D> From<Q<A, B, C, D>> for Partial<Q<A, B, C, D>>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: Q<A, B, C, D>) -> Self {
        Self::Ok(value)
    }
}

impl<const A: Precision, B, C, D> From<B> for Partial<A, B, C, D>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    fn from(value: B) -> Self {
        let outcome: Q<A, B, C, D> = value.into();
        let outcome: Result<Q<A, B, C, D>> = Ok(outcome);
        Self {
            outcome
        }
    }
}

impl<const A: Precision, B, C, D> From<Result<B>> for Partial<A, B, C, D>
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
            Ok(n) => {
                let n: Q<A, B, C, D> = n.into();
                let outcome: Result<Q<A, B, C, D>> = Ok(n);
                Self {
                    outcome
                }
            },
            Err(e) => Err::<Q<A, B, C, D>, Error>(e).into()
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Add<Q<A, B, C, E>> for Partial<A, B, D, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = ::core::ops::ControlFlow<Error, Partial<A, B, D, E>>;

    fn add(self, rhs: Q<A, B, C, E>) -> Self::Output {
        match self.outcome {
            Ok(lhs) => {
                match (lhs + rhs).outcome {
                    Ok(p) => {
                        let partial: Partial<A, B, D, E> = Ok(p).into();
                        ::core::ops::ControlFlow::Continue(partial)
                    },
                    Err(e) => {
                        ::core::ops::ControlFlow::Break(e)            
                    }
                }
            },
            Err(e) => {
                let partial: Partial<A, B, D, E> = Err(e).into();
                ::core::ops::ControlFlow::Break(d)
            }
        }
    }
}

impl<const A: Precision, B, C, D, E> ::core::ops::Sub<Q<A, B, C, E>> for Partial<A, B, D, E>
where
    B: ops::Int,
    B: ops::Prim,
    C: Mode,
    D: Mode,
    E: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Output = ::core::ops::ControlFlow<Partial<A, B, D, E>, Partial<A, B, D, E>>;
    
    #[inline]
    fn sub(self, rhs: Q<A, B, C, E>) -> Self::Output {
        match self.outcome {
            Ok(lhs) => match (lhs + rhs).outcome {
                Ok(n) => {
                    let partial: Partial<A, B, D, E> = Ok(n).into();
                    ::core::ops::ControlFlow::Continue(partial)
                },
                Err(e) => {
                    let partial: Partial<A, B, D, E> = Err::<Q<A, B, D, E>, Error>(e).into();
                    ::core::ops::ControlFlow::Break(partial) 
                }
            },
            Err(e) => {
                let partial: Partial<A, B, D, E> = Err::<Q<A, B, D, E>, Error>(e).into();
                ::core::ops::ControlFlow::Break(partial)
            }
        }
    }
}