use super::*;

pub type Result<T, Error> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum Error {
    OpsError(ops::Error),
    IllegalHex,
    AlphaOverflow,
    AlphaUnderflow
}

impl From<ops::Error> for Error {
    fn from(value: ops::Error) -> Self {
        Self::OpsError(value)
    }
}