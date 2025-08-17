use super::*;

::modwire::expose!(
    pub engine
        fold
        wide_mul
);

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B = usize, C = DefaultEngine>
where
    B: num::Int,
    C: Engine {
    v: B,
    engine: ::core::marker::PhantomData<C>
}