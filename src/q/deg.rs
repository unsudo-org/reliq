use super::*;

mode!(
    /// ```text
    /// τ = 360°
    /// ```
    /// 
    /// Represents an angular measurement (`°`).
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
    /// | `Deg1` -> `Deg<1>`   | 1         |
    /// | `Deg2` -> `Deg<2>`   | 2         |
    /// | `Deg3` -> `Deg<3>`   | 3         |
    /// | ...                              |
    /// | `Deg35` -> `Deg<35>` | 35        |
    /// | `Deg36` -> `Deg<36>` | 36        |
    /// | `Deg37` -> `Deg<37>` | 37        |
    /// ```
    Deg
);

impl<const A: u8, B> Deg<A, B> 
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<A, B>> {
        self.to_rad()?.tan()
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<A, B>> {
        self.to_rad()?.sin()
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<A, B>> {
        self.to_rad()?.cos()
    }

    #[inline]
    pub fn to_rad(self) -> Result<Rad<A, B>> {
        let ret: B = self.n;
        let ret: B = to_rad(ret)?;
        let ret: Rad<A, B> = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B> TryFrom<Rad<A, B>> for Deg<A, B>
where
    B: ops::Int,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Rad<A, B>) -> ::core::result::Result<Self, Self::Error> {
        value.to_deg()
    }
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
pub(super) fn deg_as_90<const A: u8, B>() -> Result<B>
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

#[cfg(test)]
mod test {
    use super::*;
    
    #[::rstest::rstest]
    #[case(25_00, 0_46)]
    fn tan<A, B>(#[case] angle: A, #[case] ok: B) 
    where
        A: Into<Deg2>,
        B: Into<Unit2> {
        let angle: Deg2 = angle.into();
        let ok: Unit2 = ok.into();
        let ret: Ratio2 = angle.tan().unwrap();
        assert_eq!(ret, ok);
    }

    #[::rstest::rstest]
    #[case(25_00, 0_42)]
    fn sin<A, B>(#[case] angle: A, #[case] ok: B) 
    where
        A: Into<Deg2>,
        B: Into<Unit2> {
        let angle: Deg2 = angle.into();
        let ok: Unit2 = ok.into();
        let ret: Ratio2 = angle.sin().unwrap();
        assert_eq!(ret, ok);
    }

    #[::rstest::rstest]
    #[case(1_00, 1_00)]
    fn cos<A, B>(#[case] angle: A, #[case] ok: B) 
    where
        A: Into<Deg2>,
        B: Into<Unit2> {
        let angle: Deg2 = angle.into();
        let ok: Unit2 = ok.into();
        let ret: Ratio2 = angle.cos().unwrap();
        assert_eq!(ret, ok);
    }
}