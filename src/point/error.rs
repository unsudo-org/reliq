use super::*;

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIs)]
#[derive(::thiserror::Error)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub enum Error {
    #[error("{0}")]
    QError(#[from] q::Error),
    #[error("{0}")]
    ArrayError(#[from] array::Error),
    #[error("Insufficient set size.")]
    InsufficientSetSize
}