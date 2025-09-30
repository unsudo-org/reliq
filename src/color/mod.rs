use super::*;

::modwire::expose!(
    pub hex
    pub hsl
    pub hsla
    pub mode
    pub rgb
    pub rgba
);

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
    OpsError(#[from] ops::Error),
    #[error("{0}")]
    QError(#[from] q::Error),
    #[error("Illegal hex.")]
    IllegalHex,
    #[error("Alpha overflow.")]
    AlphaOverflow,
    #[error("Alpha underflow.")]
    AlphaUnderflow,
    #[error("")]
    UnsupportedConversion
}

#[cfg_attr(feature = "std", derive(::serde::Serialize))]
#[cfg_attr(feature = "std", derive(::serde::Deserialize))]
pub struct Color<const A: u8, B = usize, C = HexMode>
where
    B: ops::Int,
    C: Mode,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    mode: C,
    m_0: ::core::marker::PhantomData<B>,
}