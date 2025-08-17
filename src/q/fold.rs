use super::*;

#[inline]
pub fn fold<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: num::Int,
    (): N<T> {
    if T::SIGNED {
        a(x, y, z)
    } else {
        b(x, y, z)
    }
}

#[inline]
fn a<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: num::Int {    
    let (x, y, z) = unsafe {
        let x: i128 = x.try_into().unwrap_unchecked();
        let y: i128 = y.try_into().unwrap_unchecked();
        let z: i128 = z.try_into().unwrap_unchecked();
        (x, y, z)
    };
    let n: i128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if n > <T as num::Int>::MAX_I128 {
        return Err(Error::Overflow)
    }
    if n < <T as num::Int>::MIN_I128 {
        return Err(Error::Underflow)
    }
    let n: T = unsafe {
        n.try_into().unwrap_unchecked()
    };
    Ok(n)
}

#[inline]
fn b<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: num::Int {
    let (x, y, z) = unsafe {
        let x: u128 = x.try_into().unwrap_unchecked();
        let y: u128 = y.try_into().unwrap_unchecked();
        let z: u128 = z.try_into().unwrap_unchecked();
        (x, y, z)
    };
    let n: u128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if n > <T as num::Int>::MAX_U128 {
        return Err(Error::Overflow)
    }
    if n < <T as num::Int>::MIN_U128 {
        return Err(Error::Underflow)
    }
    let n: T = unsafe {
        n.try_into().unwrap_unchecked()
    };
    Ok(n)
}