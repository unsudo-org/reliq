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

impl<const A: u8, B, C> TryFrom<Deg<A, B, C>> for Rad<A, B, C>
where
    B: ops::Int,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Deg<A, B, C>) -> ::core::result::Result<Self, Self::Error> {
        value.to_rad()
    }
}

impl<const A: u8, B, C> Rad<A, B, C>
where
    B: ops::Int,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<A, B, C>> {
        let ret: B = self.n;
        let ret: B = C::tan::<A, B>(ret)?;
        let ret: Unit<_, _, _> = ret.into();
        let ret: Ratio<_, _, _> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<A, B, C>> {
        let ret: B = self.n;
        let ret: B = C::sin::<A, B>(ret)?;
        let ret: Unit<_, _, _> = ret.into();
        let ret: Ratio<_, _, _> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<A, B, C>> {
        let ret: B = self.n;
        let ret: B = C::cos::<A, B>(ret)?;
        let ret: Unit<_, _, _> = ret.into();
        let ret: Ratio<_, _, _> = ret.into();
        Ok(ret)
    }

    #[inline]
    pub fn to_deg(self) -> Result<Deg<A, B, C>> {
        let ret: B = self.n;
        let ret: B = C::to_deg::<A, B>(ret)?;
        let ret: Deg<_, _, _> = ret.into();
        Ok(ret)
    }
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