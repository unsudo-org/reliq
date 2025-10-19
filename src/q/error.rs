use super::*;

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(Hash)]
#[derive(::thiserror::Error)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub enum Error {
    #[error(transparent)]
    Ops(#[from] ops::Error),

    #[error("overflow")]
    Overflow,
    
    #[error("underflow")]
    Underflow,

    #[error("division by zero")]
    DivisionByZero,

    #[error("modulo by zero")]
    ModuloByZero,

    #[error("unsupported conversion")]
    UnsupportedConversion,

    #[error("out of bounds")]
    OutOfBounds
}