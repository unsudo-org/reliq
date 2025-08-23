use super::*;

::modwire::expose!(
    pub duration
);

pub type Result<T> = ::core::result::Result<T, Error>;

pub enum Error {
    UnsupportedConversion
}

#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Timestamp<T>(T)
where
    T: ops::Int,
    T: ops::Prim,
    T: ops::Unsigned;

impl<T> From<T> for Timestamp<T>
where
    T: ops::Int,
    T: ops::Prim,
    T: ops::Unsigned {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> TryFrom<::core::time::Duration> for Timestamp<T> 
where
    T: ops::Int,
    T: ops::Prim,
    T: ops::Unsigned {
    type Error = Error;
    
    fn try_from(value: ::core::time::Duration) -> ::core::result::Result<Self, Self::Error> {
        if T::BITS_AS_U128 > 64 {
            return Err(Error::UnsupportedConversion)
        }
        let ret: u64 = value.as_secs();
        let ret: T = unsafe {
            ret.try_into().unwrap_unchecked()
        };
        let ret: Self = ret.into();
        Ok(ret)
    }
}