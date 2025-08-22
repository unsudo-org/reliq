use super::*;

pub type Result<T, Error> = ::core::result::Result<T, Error>;

pub enum Error {
    OpsError(ops::Error),
    IllegalHex,
    AlphaOverflow,
    AlphaUnderflow
}