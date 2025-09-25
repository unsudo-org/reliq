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
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIs)]
#[derive(::thiserror::Error)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub enum Error {
    #[error("{0}")]
    OpsError(#[from] ops::Error),
    #[error("Overflow.")]
    Overflow,
    #[error("Underflow.")]
    Underflow,
    #[error("Division by zero.")]
    DivisionByZero,
    #[error("Modulo by zero.")]
    ModuloByZero,
    #[error("Unsupported conversion.")]
    UnsupportedConversion,
    #[error("")]
    OutOfBounds
}