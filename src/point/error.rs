use super::*;

pub type Result<T> = ::core::result::Result<T, Error>;

pub enum Error {
    QError(q::Error),
    ArrayError(array::Error),
    InsufficientSetSize
}

impl From<q::Error> for Error {
    fn from(value: q::Error) -> Self {
        Self::QError(value)
    }
}

impl From<array::Error> for Error {
    fn from(value: array::Error) -> Self {
        Self::ArrayError(value)
    }
}