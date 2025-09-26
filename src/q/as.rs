use super::*;

macro_rules! r#as {
    ($($n:literal)*) => {
        ::paste::paste!(
            $(
                pub fn [< as _ $n >]<const A: u8, B, C>() -> Q<A, B, C>
                where
                    B: ops::Int,
                    C: Mode,
                    (): SupportedPrecision<A>,
                    (): SupportedInt<B>,
                    (): Supported<A, B>,
                    (): Supported<0, B> {
                    let ret: B = B::[< AS _ $n >];
                    let ret: B = unsafe {
                        D::cast::<0, A, _>(ret).unwrap_unchecked()
                    };
                    let ret: Q<A, B, C, D> = ret.into();
                    ret
                }
            )*
        );
    };
}

r#as!(
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32 33 34 35 36 37 38 39
    40 41 42 43 44 45 46 47 48 49
    50 51 52 53 54 55 56 57 58 59
    60 61 62 63 64 65 66 67 68 69
    70 71 72 73 74 75 76 77 78 79
    80 81 82 83 84 85 86 87 88 89
    90 91 92 93 94 95 96 97 98 99
    100
);

#[inline]
pub fn r#as<const A: u8, const B: u8, C, D, E, F, G>(n: G) -> Result<Q<B, D, E, F>> 
where
    C: ops::Int,
    D: ops::Int,
    E: Mode,
    F: Engine,
    G: Into<C>,
    (): SupportedPrecision<A>,
    (): SupportedPrecision<B>,
    (): SupportedInt<C>,
    (): SupportedInt<D>,
    (): Supported<A, C>,
    (): Supported<B, D> {
    let n: C = n.into();
    let n: u128 = n
        .try_into()
        .ok()
        .ok_or(Error::UnsupportedConversion)?;
    let n: D = n
        .try_into()
        .ok()
        .ok_or(Error::UnsupportedConversion)?;
    let n: Q<B, D, E, F> = n.into();
    Ok(n)
}

#[inline]
pub(super) fn as_180<T>() -> T
where
    T: ops::Int,
    (): SupportedInt<T> {
    let ret: u8 = 180;
    let ret: T = unsafe {
        ret.try_into().unwrap_unchecked()
    };
    ret
}