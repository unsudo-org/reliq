use super::*;

::modwire::expose!(
    pub engine
        fold
        pi
    pub precision
        scale
        wide_mul
);

pub type Q1<T> = Q<1, T>;
pub type Q2<T> = Q<2, T>;
pub type Q3<T> = Q<3, T>;
pub type Q4<T> = Q<4, T>;

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