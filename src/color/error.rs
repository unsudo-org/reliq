use super::*;

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum Error {
    OpsError(ops::Error),
    QError(q::Error),
    IllegalHex,
    AlphaOverflow,
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