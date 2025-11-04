use super::*;

impl<const A: u8, B> Q<A, B> 
where
    B: ops::Int {
    #[inline]
    fn abs(self) -> Self {
        let n: B = self.0;
        if n < B::AS_0 {
            let n: B = B::AS_0 - n;
            let ret: Self = n.into();
            return ret
        }
        self
    }
}

impl<const A: u8, B> ::core::ops::Add<B> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;
    
    #[inline]
    fn add(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = n.into();
        self + rhs
    }
}

impl<const A: u8, B> ::core::ops::Add<Result<Q<A, B>>> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Result<Q<A, B>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self + rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B> ::core::ops::Add<Q<A, B>> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Q<A, B>) -> Self::Output {
        let x: B = self.0;
        let y: B = rhs.0;
        add(x, y).map(|n| {
            n.into()
        })
    }
}

impl<const A: u8, B> ::core::ops::Sub<B> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;
    
    #[inline]
    fn sub(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = n.into();
        self - rhs
    }
}

impl<const A: u8, B> ::core::ops::Sub<Result<Q<A, B>>> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Result<Q<A, B>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self - rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B> ::core::ops::Sub<Q<A, B>> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Q<A, B>) -> Self::Output {
        let x: B = self.0;
        let y: B = rhs.0;
        sub(x, y).map(|n| {
            n.into()
        })
    }
}

impl<const A: u8, B> ::core::ops::Mul<B> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;
    
    #[inline]
    fn mul(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = n.into();
        self * rhs
    }
}

impl<const A: u8, B> ::core::ops::Mul<Result<Q<A, B>>> for Q<A, B>
where
    B: ops::Int, {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Result<Q<A, B>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self * rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B> ::core::ops::Mul<Q<A, B>> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Q<A, B>) -> Self::Output {
        let x: B = self.0;
        let y: B = rhs.0;
        mul(x, y).map(|n| {
            n.into()
        })
    }
}

impl<const A: u8, B> ::core::ops::Div<B> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;
    
    #[inline]
    fn div(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = n.into();
        self / rhs
    }
}

impl<const A: u8, B> ::core::ops::Div<Result<Q<A, B>>> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Result<Q<A, B>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self / rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B> ::core::ops::Div<Q<A, B>> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Q<A, B>) -> Self::Output {
        let x: B = self.0;
        let y: B = rhs.0;
        div(x, y).map(|n| {
            n.into()
        })
    }
}

impl<const A: u8, B> ::core::ops::Rem<B> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;
    
    #[inline]
    fn rem(self, rhs: B) -> Self::Output {
        let n: B = rhs;
        let rhs: Self = n.into();
        self % rhs
    }
}

impl<const A: u8, B> ::core::ops::Rem<Result<Q<A, B>>> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;

    #[inline]
    fn rem(self, rhs: Result<Q<A, B>>) -> Self::Output {
        match rhs {
            Ok(rhs) => self % rhs,
            Err(e) => Err(e)
        }
    }
}

impl<const A: u8, B> ::core::ops::Rem<Q<A, B>> for Q<A, B>
where
    B: ops::Int {
    type Output = Result<Self>;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        let x: B = self.0;
        let y: B = rhs.0;
        rem(x, y).map(|n| {
            n.into()
        })
    }
}

impl<const A: u8, B> ops::OverflowingAdd for Q<A, B> 
where
    B: ops::Int {
    #[inline]
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let lhs: B = self.0;
        let rhs: Self = rhs.into();
        let rhs: B = rhs.0;
        let (n, success) = lhs.overflowing_add(rhs);
        let ret: Self = n.into();
        let ret: (Self, bool) = (ret, success);
        ret
    }
}

impl<const A: u8, B> ops::OverflowingSub for Q<A, B> 
where
    B: ops::Int {
    #[inline]
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let lhs: B = self.0;
        let rhs: Self = rhs.into();
        let rhs: B = rhs.0;
        let (n, success) = lhs.overflowing_sub(rhs);
        let ret: Self = n.into();
        let ret: (Self, bool) = (ret, success);
        ret
    }
}

impl<const A: u8, B> ops::OverflowingMul for Q<A, B> 
where
    B: ops::Int {
    #[inline]
    fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let lhs: Self = self;
        let rhs: Self = rhs.into();
        if let Ok(n) = lhs * rhs {
            (n, true)
        } else {
            let ret: B = B::AS_0.into();
            let ret: Self = ret.into();
            (ret, false)
        }
    }
}

impl<const A: u8, B> ops::OverflowingDiv for Q<A, B> 
where
    B: ops::Int {
    #[inline]
    fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        let lhs: Self = self;
        let rhs: Self = rhs.into();
        if let Ok(n) = lhs / rhs {
            (n, true)
        } else {
            let ret: B = B::AS_0.into();
            let ret: Self = ret.into();
            (ret, false)
        }
    }
}

#[inline]
pub(super) fn add<T>(x: T, y: T) -> Result<T>
where
    T: ops::Int {
    x.checked_add(y).ok_or(Error::Overflow)
}

#[inline]
pub(super) fn sub<T>(x: T, y: T) -> Result<T>
where
    T: ops::Int {
    x.checked_sub(y).ok_or(Error::Underflow)
}

#[inline]
pub(super) fn mul<const A: u8, B>(x: B, y: B) -> Result<B>
where
    B: ops::Int {
    if A == 0 {
        return x.checked_mul(y).ok_or(Error::Overflow)
    }
    muldiv(x, y, scale::<A, _>())
}

#[inline]
pub(super) fn div<const A: u8, B>(x: B, y: B) -> Result<B> 
where
    B: ops::Int {
    if A == 0 {
        return x.checked_div(y).ok_or(Error::DivisionByZero)
    }
    muldiv(x, scale::<A, _>(), y)
}

#[inline]
pub(super) fn rem<const A: u8, B>(x: B, y: B) -> Result<B> 
where
    B: ops::Int {
    if y == B::AS_0 {
        return Err(Error::DivisionByZero)
    }
    let n: B = div::<A, _>(x, y)?;
    let n: B = mul::<A, _>(n, y)?;
    let n: B = sub(x, n)?;
    Ok(n)
}