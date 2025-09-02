use super::*;

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[cfg_attr(feature = "std", derive(::serde::Serialize))]
#[cfg_attr(feature = "std", derive(::serde::Deserialize))]
#[cfg_attr(feature = "std", derive(::thiserror::Error))]
pub enum Error {
    #[cfg_attr(feature = "std", error(""))]
    OpsError(ops::Error),
    #[cfg_attr(feature = "std", error(""))]
    QError(q::Error),
    #[cfg_attr(feature = "std", error(""))]
    IllegalHex,
    #[cfg_attr(feature = "std", error(""))]
    AlphaOverflow,
    #[cfg_attr(feature = "std", error(""))]
    AlphaUnderflow
}

impl From<ops::Error> for Error {
    fn from(value: ops::Error) -> Self {
        Self::OpsError(value)
    }
}

impl From<q::Error> for Error {
    fn from(value: q::Error) -> Self {
        Self::QError(value)
    }
}