use super::*;

pub mod point2d;
pub mod point3d;
pub mod point4d;
pub mod pointn;

pub type Result<T> = ::core::result::Result<T, Error>;

pub enum Error {
    QError(q::Error)
}

impl From<q::Error> for Error {
    fn from(value: q::Error) -> Self {
        Self::QError(value)
    }
}