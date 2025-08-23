use super::*;

impl<T> TryFrom<Timestamp<T>> for ::core::time::Duration 
where
    T: ops::Int,
    T: ops::Prim,
    T: ops::Unsigned {
    type Error = Error;
    
    fn try_from(value: Timestamp<T>) -> ::core::result::Result<Self, Self::Error> {
        if T::BITS_AS_U128 > 64 {
            return Err(Error::UnsupportedConversion)
        }
        let ret: T = value.0;
        let ret: u64 = unsafe {
            ret.try_into().unwrap_unchecked()
        };
        let ret: ::core::time::Duration = ::core::time::Duration::from_secs(ret);
        Ok(ret)
    }
}