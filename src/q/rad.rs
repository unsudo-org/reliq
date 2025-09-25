use super::*;

mode!(
    /// ```text
    /// τ = 2π
    /// ```
    /// 
    /// Represents an angular measurement (`ʳ`).
    /// 
    /// 
    /// # Conversion
    /// 
    /// ```text
    /// θ° = (180 / π) × θʳ
    /// θʳ = (π / 180) × θ°
    /// ```
    /// 
    /// ```text
    /// |-----------|-----------|--------|
    /// | θ = τ     | 2π        | ≈ 360° |
    /// | θ = π     |  π        | ≈ 180° |
    /// | θ = π / 2 |  π / 2    | ≈ 90°  |
    /// ```
    /// 
    /// 
    /// # Where
    /// 
    /// - `θ°` is the angle in degrees.
    /// - `θʳ` is the angle in radians.
    /// 
    /// 
    /// # Variant
    /// 
    /// ```text
    /// | Alias                | Precision |
    /// |----------------------|-----------|
    /// | `Rad1` -> `Rad<1>`   | 1         |
    /// | `Rad2` -> `Rad<2>`   | 2         |
    /// | `Rad3` -> `Rad<3>`   | 3         |
    /// | ...                              |
    /// | `Rad35` -> `Rad<35>` | 35        |
    /// | `Rad36` -> `Rad<36>` | 36        |
    /// | `Rad37` -> `Rad<37>` | 37        |
    /// ```
    Rad
);

impl<const A: u8, B> TryFrom<Deg<A, B>> for Rad<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;
    
    #[inline]
    fn try_from(value: Deg<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let n: Deg<_, _> = value;
        let n: Rad<_, _> = n.to_rad()?;
        Ok(n)
    }
}

impl<const A: u8, B> Rad<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<A, B>> {
        let x: Ratio<_, _> = Self::sin(self)?;
        let y: Ratio<_, _> = Self::cos(self)?;

    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<A, B>> {
        let angle: B = self.n;
        let n: B = sub(to_rad(deg90()?)?, angle)?;
        let n: Self = n.into();
        Self::cos(n)
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<A, B>> {
        let angle: B = self.n;
        let scale: B = scale::<A, _>();
        let pi_0: B = pi::<A, _>();
        let pi_1: B = pi_0.checked_mul(B::AS_2).ok_or(Error::Overflow)?;
        let mut n: B = angle % pi_1;
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
        let ret: Ratio<A, B> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn to_deg(self) -> Result<Deg<A, B>> {
        let ret: B = self.n;
        let ret: B = to_deg::<A, _>(ret)?;
        let ret: Deg<_, _> = ret.into();
        Ok(ret)
    }
}

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

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00, 0_84)]
fn test_sin<A, B>(#[case] angle: A, #[case] ok: B) 
where
    A: Into<Rad2>,
    B: Into<Unit2> {
    let angle: Rad2 = angle.into();
    let ok: Unit2 = ok.into();
    let ret: Ratio2 = angle.sin().unwrap();
    assert_eq!(ret, ok);
}

#[cfg(test)]
#[::rstest::rstest]
#[case(1_00, 0_54)]
fn test_cos<A, B>(#[case] angle: A, #[case] ok: B) 
where
    A: Into<Rad2>,
    B: Into<Unit2> {
    let angle: Rad2 = angle.into();
    let ok: Unit2 = ok.into();
    let ret: Ratio2 = angle.cos().unwrap();
    assert_eq!(ret, ok);
}