#![no_std]

pub mod ops;
pub mod q;

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