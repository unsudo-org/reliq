use super::*;

type F<T> = T;

pub struct DefaultEngine; 

pub trait Engine {
    #[inline]
    fn add<T>(x: F<T>, y: F<T>) -> Result<F<T>>
    where
        T: num::Int {
        x.checked_add(y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(x: F<T>, y: F<T>) -> Result<F<T>>
    where
        T: num::Int {
        x.checked_sub(y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<T>(X: F<T>, y: F<T>) -> Result<F<T>>
    where
        T: num::Int {
        Ok(Self::muldiv(x, y, s))
    }

    #[inline]
    fn muldiv<T>(x: T, y: T, z: T) -> Result<T> 
    where 
        T: num::Int {
        if z == T::AS_0 {
            return Err(Error::DivisionByZero);
        }
        match (T::BITS_U128, T::SIGNED) {
            (_, true) if T::BITS_U128 <= 64 => {
                let n: T = x.checked_mul(y).ok_or(Error::Overflow)?;
                let n: T = n / z;
                Ok(n)
            },
            (_, false) if T::BITS_U128 < 128 => {
                let (a, b) = wide_mul(x, y)?;
                if b >= z {
                    Err(Error::Overflow)
                } else if b == T::AS_0 {
                    Ok(a / z)
                } else {
                    Ok(fold(a, b, z)? / z)
                }
            },
            (128, _) => {
                let n: T = x.checked_mul(y).ok_or(Error::Overflow)?;
                Ok(n / z)
            },
            _ => unsafe {
                ::core::hint::unreachable_unchecked();
            }
        }
    }
}

impl Engine for DefaultEngine {}