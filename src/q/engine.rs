use super::*;

type F<T> = T;
type Rad<T> = T;
type Deg<T> = T;
type Ratio<T> = T;

#[derive(Clone)]
#[derive(Copy)]
pub struct DefaultEngine; 

impl Engine for DefaultEngine {}

pub trait Engine 
where
    Self: Sized,
    Self: Clone,
    Self: Copy {
    #[inline]
    fn tan<const A: u8, B>(angle: Rad<F<B>>) -> Result<Ratio<F<B>>>
    where
        B: ops::Int,
        B: ops::Prim,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        Self::div(Self::sin(angle)?, Self::cos(angle)?)
    }

    #[inline]
    fn sin<const A: u8, B>(angle: Rad<F<B>>) -> Result<Ratio<F<B>>>
    where
        B: ops::Int,
        B: ops::Prim,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        Self::cos(Self::sub(Self::to_rad::<A, B>(deg90()?)?, angle)?)
    }

    #[inline]
    fn cos<const A: u8, B>(angle: Rad<F<B>>) -> Result<Ratio<F<B>>>
    where
        B: ops::Int,
        B: ops::Prim,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let (scale, pi, pi_2) = {
            let scale: B = scale::<A, _>();
            let pi: B = pi::<A, _>();
            let pi_2: B = pi.checked_mul(B::AS_2).ok_or(Error::Overflow)?;
            (scale, pi, pi_2)
        };
        let mut n: B = angle % pi_2;
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
            term = Self::muldiv(term, n, scale)?;
            term = Self::muldiv(term, n, scale)?;
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
    fn to_rad<const A: u8, B>(angle: Deg<F<B>>) -> Result<Rad<F<B>>>
    where
        B: ops::Int,
        B: ops::Prim,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        Self::muldiv(angle, pi::<A, _>(), n180::<B>() * scale::<A, B>())
    }

    #[inline]
    fn to_deg<const A: u8, B>(angle: Rad<F<B>>) -> Result<Deg<F<B>>>
    where
        B: ops::Int,
        B: ops::Prim,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        Self::muldiv(angle, n180::<B>() * scale::<A, B>(), pi())
    }

    #[inline]
    fn cast<const A: u8, const B: u8, C>(n: F<C>) -> Result<F<C>>
    where
        C: ops::Int,
        C: ops::Prim,
        (): SupportedPrecision<A>,
        (): SupportedPrecision<B>,
        (): SupportedInt<C>,
        (): Supported<A, C>,
        (): Supported<B, C> {
        let old_scale: C = scale::<A, _>();
        let new_scale: C = scale::<B, _>();
        Self::muldiv(n, new_scale, old_scale)
    }

    #[inline]
    fn to_negative<T>(n: F<T>) -> F<T>
    where
        T: ops::Int,
        T: ops::Prim,
        T: ops::Signed,
        (): SupportedInt<T> {
        if n == T::AS_0 {
            n
        } else {
            T::AS_0 - n
        }
    }

    #[inline]
    fn to_positive<T>(n: F<T>) -> F<T>
    where
        T: ops::Int,
        T: ops::Prim,
        T: ops::Signed,
        (): SupportedInt<T> {
        if n >= T::AS_0 {
            n
        } else {
            T::AS_0 - n
        }
    }
    
    #[inline]
    fn add<T>(x: F<T>, y: F<T>) -> Result<F<T>>
    where
        T: ops::Int,
        T: ops::Prim,
        (): SupportedInt<T> {
        x.checked_add(y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(x: F<T>, y: F<T>) -> Result<F<T>>
    where
        T: ops::Int,
        T: ops::Prim,
        (): SupportedInt<T> {
        x.checked_sub(y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<const A: u8, B>(x: F<B>, y: F<B>) -> Result<F<B>>
    where
        B: ops::Int,
        B: ops::Prim,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        if A == 0 {
            return x.checked_mul(y).ok_or(Error::Overflow)
        }
        Self::muldiv(x, y, scale::<A, _>())
    }

    #[inline]
    fn div<const A: u8, B>(x: F<B>, y: F<B>) -> Result<F<B>>
    where
        B: ops::Int,
        B: ops::Prim,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        if A == 0 {
            return x.checked_div(y).ok_or(Error::DivisionByZero)
        }
        Self::muldiv(x, scale::<A, _>(), y)
    }

    #[inline]
    fn muldiv<T>(x: T, y: T, z: T) -> Result<T> 
    where 
        T: ops::Int,
        T: ops::Prim,
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

#[inline]
fn wide_mul<T>(x: T, y: T) -> Result<(T, T)>
where 
    T: ops::Int,
    T: ops::Prim,
    (): SupportedInt<T> {
    if T::SIGNED {
        signed_wide_mul(x, y)
    } else {
        unsigned_wide_mul(x, y)
    }
}

#[inline]
fn signed_wide_mul<T>(x: T, y: T) -> Result<(T, T)> 
where 
    T: ops::Int,
    T: ops::Prim,
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
    T: ops::Prim,
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
    T: ops::Prim,
    (): SupportedInt<T> {
    if T::SIGNED {
        signed_fold(x, y, z)
    } else {
        unsigned_fold(x, y, z)
    }
}

#[inline]
fn signed_fold<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: ops::Int,
    T: ops::Prim,
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
    T: ops::Prim,
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
fn n180<T>() -> T
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
pub fn deg90<const A: u8, B>() -> Result<Deg<B>>
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