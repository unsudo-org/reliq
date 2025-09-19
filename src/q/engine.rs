use super::*;

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct DefaultEngine;

impl Engine for DefaultEngine {}

pub trait Engine 
where
    Self: Clone,
    Self: Copy {
    #[inline]
    fn tan<const A: Precision, B>(rad_angle: B) -> Result<B>
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let x: B = Self::sin::<A, B>(rad_angle)?;
        let y: B = Self::cos::<A, B>(rad_angle)?;
        Ok(Self::div::<A, B>(x, y)?.into())
    }

    #[inline]
    fn sin<const A: Precision, B>(rad_angle: B) -> Result<B>
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        Self::cos::<A, B>(Self::sub::<B>(Self::to_rad::<A, B>(deg90::<A, B>()?)?, rad_angle)?)
    }

    #[inline]
    fn cos<const A: Precision, B>(rad_angle: B) -> Result<B>
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let (scale, pi, pi_2) = {
            let scale: B = scale::<A, B>();
            let pi: B = pi::<A, B>();
            let pi_2: B = pi.checked_mul(B::AS_2).ok_or(Error::Overflow)?;
            (scale, pi, pi_2)
        };
        let mut n: B = rad_angle % pi_2;
        if n < B::AS_0 {
            n = n.checked_add(pi_2).ok_or(Error::Overflow)?;
        }
        if n > pi {
            n = n.checked_sub(pi_2).ok_or(Error::Underflow)?;
        }
        let mut ret: B = scale;
        let mut term: B = scale;
        let mut sign: bool = true;
        let mut k: B = B::AS_1;
        loop {
            let f: B = (B::AS_2 * k - B::AS_1) * (B::AS_2 * k);
            term = Self::muldiv::<B>(term, n, scale)?;
            term = Self::muldiv::<B>(term, n, scale)?;
            term = term.checked_div(f).ok_or(Error::DivisionByZero)?;
            if term == B::AS_0 {
                break
            }
            ret = if sign {
                ret.checked_sub(term).ok_or(Error::Underflow)?
            } else {
                ret.checked_add(term).ok_or(Error::Overflow)?
            };
            sign = !sign;
            k = k.checked_add(B::AS_1).ok_or(Error::Overflow)?;
        }
        Ok(ret)
    }

    #[inline]
    fn to_rad<const A: Precision, B>(deg_angle: B) -> Result<B>
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        Self::muldiv(deg_angle, pi::<A, B>(), as_180::<B>() * scale::<A, B>())
    }

    #[inline]
    fn to_deg<const A: Precision, B>(rad_angle: B) -> Result<B>
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        Self::muldiv::<B>(rad_angle, as_180::<B>() * scale::<A, B>(), pi::<A, B>())
    }

    #[inline]
    fn round_up<const A: Precision, B>(n: B) -> B
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let scale: B = scale::<A, B>();
        if n % scale == B::AS_0 {
            return n
        }
        if n >= B::AS_0 {
            let n: B = n / scale;
            let n: B = n + B::AS_1;
            let n: B = n * scale;
            return n
        }
        let n: B = n / scale;
        let n: B = n * scale;
        n
    }

    fn round_down<const A: Precision, B>(n: B) -> B 
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let scale: B = scale::<A, B>();
        if n % scale == B::AS_0 {
            return n
        }
        if n >= B::AS_0 {
            let n: B = n / scale;
            let n: B = n * scale;
            return n
        }
        let n: B = n / scale;
        let n: B = n - B::AS_1;
        let n: B = n * scale;
        n
    }

    fn round_towards_zero<const A: Precision, B>(n: B) -> B 
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let scale: B = scale::<A, B>();
        let n: B = n / scale;
        let n: B = n * scale;
        n
    }

    fn round_away_from_zero<const A: Precision, B>(n: B) -> B
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let scale: B = scale::<A, B>();
        if n % scale == B::AS_0 {
            return n
        }
        if n < B::AS_0 {
            let n: B = n / scale;
            let n: B = n * scale;
            let n: B = n - scale;
            return n
        }
        let n: B = n / scale;
        let n: B = n * scale;
        let n: B = n + scale;
        n
    }

    #[inline]
    fn to_negative<T>(n: T) -> T
    where
        T: ops::Int,
        T: ops::Signed,
        (): SupportedInt<T> {
        if n == T::AS_0 {
            return n
        }
        T::AS_0 - n    
    }

    #[inline]
    fn to_positive<T>(n: T) -> T
    where
        T: ops::Int,
        T: ops::Signed,
        (): SupportedInt<T> {
        if n >= T::AS_0 {
            return n
        }
        T::AS_0 - n    
    }

    #[inline]
    fn cast<const A: Precision, const B: Precision, C>(n: C) -> Result<lossy::Lossy<C>>
    where
        C: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedPrecision<B>,
        (): SupportedInt<C>,
        (): Supported<A, C>,
        (): Supported<B, C> {
        let old_scale: C = scale::<A, C>();
        let new_scale: C = scale::<B, C>();
        let n: C = Self::muldiv::<C>(n, new_scale, old_scale)?;
        if B < A {
            Ok(lossy::Lossy::Trunc(n))
        } else {
            Ok(lossy::Lossy::Exact(n))
        }
    }

    #[inline]
    fn lerp<const A: Precision, B>(x: B, y: B, t: B) -> Result<B>
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let d: B = Self::sub::<B>(y, x)?;
        let s: B = Self::muldiv::<B>(d, t, scale::<A, B>())?;
        let ret: B = Self::add::<B>(x, s)?;
        Ok(ret)
    }

    #[inline]
    fn sqrt<const A: Precision, B>(n: B) -> Result<B>
    where
        B: ops::Int,
        B: ops::Signed,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        if n < B::AS_0 {
            return Err(Error::Underflow)
        }
        if n == B::AS_0 || n == B::AS_1 {
            return Ok(n)
        }
        let mut ret: B = n.checked_div(B::AS_2).ok_or(Error::DivisionByZero)?;
        let mut last: B;
        loop {
            last = ret;
            ret = Self::add::<B>(ret, Self::div::<A, B>(n, ret)?)?;
            ret = Self::div::<A, B>(ret, B::AS_2)?;
            if ret == last || ret == last.checked_add(B::AS_1).unwrap_or(ret) {
                break
            }
        }
        Ok(ret)
    }

    #[inline]
    fn add<T>(x: T, y: T) -> Result<T>
    where
        T: ops::Int,
        (): SupportedInt<T> {
        x.checked_add(y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(x: T, y: T) -> Result<T>
    where
        T: ops::Int,
        (): SupportedInt<T> {
        x.checked_sub(y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<const A: u8, B>(x: B, y: B) -> Result<B>
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        if A == 0 {
            return x.checked_mul(y).ok_or(Error::Overflow)
        }
        Self::muldiv::<B>(x, y, scale::<A, B>())
    }

    #[inline]
    fn div<const A: u8, B>(x: B, y: B) -> Result<B>
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        if A == 0 {
            return x.checked_div(y).ok_or(Error::DivisionByZero)
        }
        Self::muldiv::<B>(x, scale::<A, B>(), y)
    }

    #[inline]
    fn rem<const A: Precision, B>(x: B, y: B) -> Result<B> 
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        if y == B::AS_0 {
            return Err(Error::DivisionByZero)
        }
        let n: B = Self::div::<A, B>(x, y)?;
        let n: B = Self::mul::<A, B>(n, y)?;
        let n: B = Self::sub::<B>(x, n)?;
        Ok(n)
    }

    #[inline]
    fn muldiv<T>(x: T, y: T, z: T) -> Result<T> 
    where 
        T: ops::Int,
        (): SupportedInt<T> {
        if z == T::AS_0 {
            return Err(Error::DivisionByZero);
        }
        match (T::BITS_AS_U128, T::SIGNED) {
            (_, true) if T::BITS_AS_U128 <= 64 => {
                let n: T = x.checked_mul(y).ok_or(Error::Overflow)?;
                let n: T = n / z;
                Ok(n)
            },
            (_, false) if T::BITS_AS_U128 < 128 => {
                let (a, b) = wide_mul::<T>(x, y)?;
                if b >= z {
                    Err(Error::Overflow)
                } else if b == T::AS_0 {
                    Ok(a / z)
                } else {
                    Ok(fold::<T>(a, b, z)? / z)
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

#[inline]
fn wide_mul<T>(x: T, y: T) -> Result<(T, T)>
where 
    T: ops::Int,
    (): SupportedInt<T> {
    if T::SIGNED {
        signed_wide_mul::<T>(x, y)
    } else {
        unsigned_wide_mul::<T>(x, y)
    }
}

#[inline]
fn signed_wide_mul<T>(x: T, y: T) -> Result<(T, T)> 
where 
    T: ops::Int,
    (): SupportedInt<T> {    
    assert!(T::SIGNED);
    assert!(T::BITS_AS_U128 <= 64);
    let a: T = T::BITS;
    let b: T = T::AS_2;
    let n: T = a / b;
    let mask: T = (T::AS_1 << n) - T::AS_1;
    let (lo_lo, lo_hi, hi_lo, hi_hi) = {
        let x_lo: T = x & mask;
        let x_hi: T = x >> n;
        let y_lo: T = y & mask;
        let y_hi: T = y >> n;
        let lo_lo: T = x_lo.checked_mul(y_lo).ok_or(Error::Overflow)?;
        let lo_hi: T = x_lo.checked_mul(y_hi).ok_or(Error::Overflow)?;
        let hi_lo: T = x_hi.checked_mul(y_lo).ok_or(Error::Overflow)?;
        let hi_hi: T = x_hi.checked_mul(y_hi).ok_or(Error::Overflow)?;
        (lo_lo, lo_hi, hi_lo, hi_hi)
    };
    let a: T = lo_hi.checked_add(hi_lo).ok_or(Error::Overflow)?;
    let b: T = a << n;
    let hi: T = if lo_lo > lo_lo.wrapping_add(b) {
        T::AS_1
    } else {
        T::AS_0
    };
    let hi: T = hi_hi
        .checked_add(a >> n)
        .ok_or(Error::Overflow)?
        .checked_add(hi)
        .ok_or(Error::Overflow)?;
    let lo: T = lo_lo.wrapping_add(b);
    Ok((lo, hi))
}

#[inline]
fn unsigned_wide_mul<T>(x: T, y: T) -> Result<(T, T)> 
where 
    T: ops::Int,
    (): SupportedInt<T> {
    assert!(!T::SIGNED);
    assert!(T::BITS_AS_U128 <= 64);
    let (x, y) = unsafe {
        let x: u128 = x.try_into().unwrap_unchecked();
        let y: u128 = y.try_into().unwrap_unchecked();
        (x, y)
    };
    let (a, b) = {
        let x_hi: u128 = x >> 64;
        let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
        let y_hi: u128 = y >> 64;
        let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
        let lo_lo: u128 = x_lo * y_lo;
        let lo_hi: u128 = x_lo * y_hi;
        let hi_lo: u128 = x_hi * y_lo;
        let hi_hi: u128 = x_hi * y_hi;
        let m: u128 = lo_hi + hi_lo;
        let c: u128 = ((m < lo_hi) as u128) << 64;
        let m_lo: u128 = m << 64;
        let m_hi: u128 = m >> 64;
        let a: u128 = lo_lo.wrapping_add(m_lo);
        let b: u128 = hi_hi + m_hi + c + ((a < lo_lo) as u128);
        (a, b)
    };
    if T::BITS_AS_U128 == 128 {
        unsafe {
            let a: T = a.try_into().unwrap_unchecked();
            let b: T = b.try_into().unwrap_unchecked();
            return Ok((a, b))
        }
    }
    if a > T::MAX_AS_U128 {
        return Err(Error::Overflow)
    }
    if a < T::MIN_AS_U128 {
        return Err(Error::Underflow)
    }
    if b > T::MAX_AS_U128 {
        return Err(Error::Overflow)
    }
    if b < T::MIN_AS_U128 {
        return Err(Error::Underflow)
    }
    let (a, b) = unsafe {
        let a: T = a.try_into().unwrap_unchecked();
        let b: T = b.try_into().unwrap_unchecked();
        (a, b)
    };
    Ok((a, b))
}

#[inline]
fn fold<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: ops::Int,
    (): SupportedInt<T> {
    if T::SIGNED {
        signed_fold::<T>(x, y, z)
    } else {
        unsigned_fold::<T>(x, y, z)
    }
}

#[inline]
fn signed_fold<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: ops::Int,
    (): SupportedInt<T> {    
    let (x, y, z) = unsafe {
        let x: i128 = x.try_into().unwrap_unchecked();
        let y: i128 = y.try_into().unwrap_unchecked();
        let z: i128 = z.try_into().unwrap_unchecked();
        (x, y, z)
    };
    let n: i128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if n > T::MAX_AS_I128 {
        return Err(Error::Overflow)
    }
    if n < T::MIN_AS_I128 {
        return Err(Error::Underflow)
    }
    let n: T = unsafe {
        n.try_into().unwrap_unchecked()
    };
    Ok(n)
}

#[inline]
fn unsigned_fold<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: ops::Int,
    (): SupportedInt<T> {
    let (x, y, z) = unsafe {
        let x: u128 = x.try_into().unwrap_unchecked();
        let y: u128 = y.try_into().unwrap_unchecked();
        let z: u128 = z.try_into().unwrap_unchecked();
        (x, y, z)
    };
    let n: u128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if n > T::MAX_AS_U128 {
        return Err(Error::Overflow)
    }
    if n < T::MIN_AS_U128 {
        return Err(Error::Underflow)
    }
    let n: T = unsafe {
        n.try_into().unwrap_unchecked()
    };
    Ok(n)
}

#[inline]
fn as_180<T>() -> T
where
    T: ops::Int,
    (): SupportedInt<T> {
    let ret: u8 = 180;
    let ret: T = unsafe {
        ret.try_into().unwrap_unchecked()
    };
    ret
}

#[inline]
fn deg90<const A: u8, B>() -> Result<B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    let deg: B = if B::SIGNED {
        let n: i128 = 90;
        let n: B = unsafe {
            n.try_into().unwrap_unchecked()
        };
        n
    } else {
        let n: u128 = 90;
        let n: B = unsafe {
            n.try_into().unwrap_unchecked()
        };
        n
    };
    let ret: B = deg.checked_mul(scale()).ok_or(Error::Overflow)?;
    Ok(ret)
}