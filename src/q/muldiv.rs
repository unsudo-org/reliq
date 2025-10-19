use super::*;

pub(super) fn muldiv<T>(
    x: T, 
    y: T, 
    z: T
) -> Result<T> 
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

fn wide_mul<T>(x: T, y: T) -> Result<(T, T)>
where 
    T: ops::Int,
    (): SupportedInt<T> {
    if T::SIGNED {
        signed_wide_mul(x, y)
    } else {
        unsigned_wide_mul(x, y)
    }
}

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

fn fold<T>(
    x: T, 
    y: T, 
    z: T
) -> Result<T> 
where 
    T: ops::Int,
    (): SupportedInt<T> {
    if T::SIGNED {
        signed_fold(x, y, z)
    } else {
        unsigned_fold(x, y, z)
    }
}

fn signed_fold<T>(
    x: T, 
    y: T, 
    z: T
) -> Result<T> 
where 
    T: ops::Int,
    (): SupportedInt<T> {
    assert!(T::SIGNED);
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

fn unsigned_fold<T>(
    x: T, 
    y: T, 
    z: T
) -> Result<T> 
where 
    T: ops::Int,
    (): SupportedInt<T> {
    assert!(!T::SIGNED);
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