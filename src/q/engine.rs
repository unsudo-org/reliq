use super::*;

type F<T> = T;
type Deg<T> = T;
type Rad<T> = T;

pub struct DefaultEngine; 

pub trait Engine {
    #[inline]
    fn to_rad<const A: u8, B>(angle: Deg<B>) -> Result<Rad<B>>
    where
        B: num::Int,
        (): Precision<A>,
        (): N<B> {
        Ok(Self::muldiv(angle, pi::<A, _>(), n180::<B>() * scale::<A, B>())?)
    }

    #[inline]
    fn to_deg<const A: u8, B>(angle: Rad<B>) -> Result<Deg<B>>
    where
        B: num::Int,
        (): Precision<A>,
        (): N<B> {
        Ok(Self::muldiv(angle, n180::<B>() * scale::<A, B>(), pi())?)
    }

    #[inline]
    fn to_negative<T>(n: F<T>) -> T
    where
        T: num::Int,
        (): N<T> {
        if n == T::AS_0 {
            n
        } else {
            T::AS_0 - n
        }
    }

    #[inline]
    fn to_positive<T>(n: F<T>) -> T
    where
        T: num::Int,
        (): N<T> {
        if n >= T::AS_0 {
            n
        } else {
            T::AS_0 - n
        }
    }

    #[inline]
    fn add<T>(x: F<T>, y: F<T>) -> Result<F<T>>
    where
        T: num::Int,
        (): N<T> {
        x.checked_add(y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(x: F<T>, y: F<T>) -> Result<F<T>>
    where
        T: num::Int,
        (): N<T> {
        x.checked_sub(y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<const A: u8, B>(x: F<B>, y: F<B>) -> Result<F<B>>
    where
        B: num::Int,
        (): Precision<A>,
        (): N<B> {
        Ok(Self::muldiv(x, y, scale::<A, _>())?)
    }

    #[inline]
    fn div<const A: u8, B>(x: F<B>, y: F<B>) -> Result<F<B>>
    where
        B: num::Int,
        (): Precision<A>,
        (): N<B> {
        let ret: u128 = scale::<A, u128>();
        if ret.is_power_of_two() {
            let ret: B = x << ret.trailing_zeros().try_into().unwrap();
            Ok(ret)
        } else {
            Ok(Self::muldiv(x, scale::<A, _>(), y)?)
        }
    }

    #[inline]
    fn muldiv<T>(x: T, y: T, z: T) -> Result<T> 
    where 
        T: num::Int,
        (): N<T> {
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