use super::*;

pub mod point2d;
pub mod point3d;
pub mod point4d;
pub mod pointn;

pub use error::*;
pub use point::*;

mod error {
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
}

mod point {
    use super::*;

    pub trait Point<const A: u8, B, C>
    where
        B: ops::Int,
        B: ops::Prim,
        B: ops::Signed,
        C: q::Engine,
        (): q::SupportedPrecision<A>,
        (): q::SupportedInt<B>,
        (): q::Supported<A, B> {
        fn distance_between(self, rhs: Self) -> Result<q::Q<A, B, q::DefaultMode, C>>;
    }
}