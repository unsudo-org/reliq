use super::*;

impl<const A: u8, B> Q<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Q<A, B>> 
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let n: B = self.0;
        let n: B = tan::<A, _>(n)?;
        let n: Self = n.into();
        Ok(n)
    }

    #[inline]
    pub fn sin(self) -> Result<Q<A, B>> 
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let n: B = self.0;
        let n: B = sin::<A, _>(n)?;
        let n: Self = n.into();
        Ok(n)
    }

    #[inline]
    pub fn cos(self) -> Result<Q<A, B>> 
    where
        B: ops::Int,
        (): SupportedPrecision<A>,
        (): SupportedInt<B>,
        (): Supported<A, B> {
        let n: B = self.0;
        let n: B = cos::<A, _>(n)?;
        let n: Self = n.into();
        Ok(n)
    }
}

#[inline]
pub(super) fn tan<const A: u8, B>(rad_angle: B) -> Result<B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    let x: B = sin::<A, _>(rad_angle)?;
    let y: B = cos::<A, _>(rad_angle)?;
    let n: B = div::<A, _>(x, y)?;
    Ok(n)
}

#[inline]
pub(super) fn sin<const A: u8, B>(rad_angle: B) -> Result<B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    cos::<A, _>(sub(to_rad::<A, _>(deg_as_90::<A, _>()?)?, rad_angle)?)
}

#[inline]
pub(super) fn cos<const A: u8, B>(rad_angle: B) -> Result<B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    let scale: B = scale::<A, _>();
    let pi_0: B = pi::<A, _>();
    let pi_1: B = pi_0.checked_mul(B::AS_2).ok_or(Error::Overflow)?;
    let mut n: B = rad_angle % pi_1;
    if n < B::AS_0 {
        n = n.checked_add(pi_1).ok_or(Error::Overflow)?;
    }
    if n > pi_0 {
        n = n.checked_sub(pi_1).ok_or(Error::Underflow)?;
    }
    let mut ret: B = scale;
    let mut term: B = scale;
    let mut sign: bool = true;
    let mut k: B = B::AS_1;
    loop {
        let f: B = (B::AS_2 * k - B::AS_1) * (B::AS_2 * k);
        term = muldiv(term, n, scale)?;
        term = muldiv(term, n, scale)?;
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

pub(super) fn to_deg<const A: u8, B>(rad_angle: B) -> Result<B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    muldiv(rad_angle, as_180() * scale(), pi())
}

#[inline]
pub(super) fn to_rad<const A: u8, B>(deg_angle: B) -> Result<B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    muldiv(deg_angle, pi(), as_180() * scale())
}

#[inline]
fn deg_as_90<const A: u8, B>() -> Result<B>
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