use super::*;

::modwire::expose!(
    pub r#as
    pub core
    pub core_fmt
    pub default_mode
    pub deg_mode
    pub engine
    pub mode
        pi
    pub rad_mode
        scale
    pub supported
);

type Ratio<T> = T;

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero,
    RemByZero
}

#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B = usize, C = DefaultMode, D = DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    n: B,
    m_0: ::core::marker::PhantomData<C>,
    m_1: ::core::marker::PhantomData<D>
}


#[cfg(test)]
mod test {
    use super::*;

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into(), 2_00.into())]
    fn add(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = (x + y).unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into(), 0_00.into())]
    fn sub(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = (x - y).unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into(), 1_00.into())]
    #[case(0_50.into(), 0_25.into(), 0_12.into())]
    fn mul(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = (x * y).unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into(), 1_00.into())]
    fn div(#[case] x: Q2<u32>, #[case] y: Q2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = (x / y).unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(25_00.into(), 0_46.into())]
    fn tan(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = angle.tan().unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(25_00.into(), 0_42.into())]
    fn sin(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = angle.sin().unwrap();
        assert_eq!(ret, expected);
    }

    #[::rstest::rstest]
    #[case(1_00.into(), 1_00.into())]
    fn cos(#[case] angle: Deg2<u32>, #[case] expected: Q2<u32>) {
        let ret: Q2<_> = angle.cos().unwrap();
        assert_eq!(ret, expected);
    }
}