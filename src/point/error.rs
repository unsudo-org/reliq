use super::*;

pub type Result<T> = ::core::result::Result<T, Error>;

pub enum Error {
    QError(q::Error)
}

impl From<q::Error> for Error {
    fn from(value: q::Error) -> Self {
        Self::QError(value)
    }
}