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

impl<const A: u8, B, C> Deg<A, B, C> 
where
    B: ops::Int,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    pub fn tan(self) -> Result<Ratio<A, B, C>> {
        self.to_rad()?.tan()
    }

    #[inline]
    pub fn sin(self) -> Result<Ratio<A, B, C>> {
        self.to_rad()?.sin()
    }

    #[inline]
    pub fn cos(self) -> Result<Ratio<A, B, C>> {
        self.to_rad()?.cos()
    }

    #[inline]
    pub fn to_rad(self) -> Result<Rad<A, B, C>> {
        let ret: B = self.n;
        let ret: B = C::to_rad(ret)?;
        let ret: Rad<A, B, C> = ret.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C> TryFrom<Rad<A, B, C>> for Deg<A, B, C>
where
    B: ops::Int,
    C: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    type Error = Error;

    fn try_from(value: Rad<A, B, C>) -> ::core::result::Result<Self, Self::Error> {
        value.to_deg()
    }
}

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
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