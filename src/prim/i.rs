use super::*;

#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct I<T>(T)
where
    T: num::Int,
    T: num::Prim;

impl<T> ::core::ops::Add for I<T>
where
    T: num::Int,
    T: num::Prim {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0.checked_add(rhs.0).map(I).ok_or(Error::Overflow)
    }
}

impl<T> ::core::ops::Sub for I<T>
where
    T: num::Int,
    T: num::Prim {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0.checked_sub(rhs.0).map(I).ok_or(Error::Underflow)
    }
}

impl<T> ::core::ops::Mul for I<T>
where
    T: num::Int,
    T: num::Prim {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.0.checked_mul(rhs.0).map(I).ok_or(Error::Overflow)
    }
}

impl<T> ::core::ops::Div for I<T>
where
    T: num::Int,
    T: num::Prim {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.0.checked_div(rhs.0).map(I).ok_or(Error::DivisionByZero)
    }
}

impl<T> ::core::ops::Rem for I<T>
where
    T: num::Int,
    T: num::Prim {
    type Output = Result<Self>;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        self.0.checked_rem(rhs.0).map(I).ok_or(Error::)
    }
}

impl<T> ::core::ops::Neg for I<T> 
where
    T: num::Int,
    T: num::Prim {
    type Output = Result<Self>;

    #[inline]
    fn neg(self) -> Self::Output {
        self.0.checked_neg().map(I).ok_or()
    }
}

impl<T> ::core::ops::Shl for I<T>
where
    T: num::Int,
    T: num::Prim {
    type Output = Result<Self>;

    #[inline]
    fn shl(self, rhs: Self) -> Self::Output {
        let rhs: u32 = rhs.try_into().ok().ok_or()?;
        self.0.checked_shr(rhs).map(I).ok_or()
    }
}

impl<T> ::core::ops::Shr for I<T>
where
    T: num::Int,
    T: num::Prim {
    type Output = Result<Self>;

    #[inline]
    fn shr(self, rhs: Self) -> Self::Output {
        
    }
}

