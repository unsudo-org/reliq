use super::*;

type Deg<T> = T;

#[inline]
pub fn deg90<const A: u8, B>() -> Result<Deg<B>>
where
    B: num::Int,
    (): Precision<A>,
    (): N<B> {
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