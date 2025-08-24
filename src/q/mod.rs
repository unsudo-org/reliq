use super::*;

::modwire::expose!(
    pub r#as
    pub core
    pub core_fmt
    pub default_mode
    pub deg_mode
    pub engine
    pub mode
    pub pi
    pub rad_mode
    pub scale
    pub supported
);

type Ratio<T> = T;
type Precision = u8;

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero,
    ModuloByZero
}

#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: Precision, B = usize, C = DefaultMode, D = DefaultEngine>
where
    B: ops::Int,
    B: ops::Prim,
    D: Engine,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    n: B,
    mode: ::core::marker::PhantomData<C>,
    engine: ::core::marker::PhantomData<D>
}