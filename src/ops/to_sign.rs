use super::*;

pub trait ToSign {
    fn to_signed<T>(self) -> Result<T>
    where
        T: Signed;
    
    fn to_unsigned<T>(self) -> Result<T>
    where
        T: Unsigned;
}

impl ToSign for u8 {
    fn to_signed<T>(self) -> Result<T>
    where
        T: Int,
        T: Signed {
        let n: u128 = self.into();
        if n > i8::MAX_AS_U128 {
            return Err(Error::Overflow)
        }
        if n > T::MAX_AS_U128 {
            return Err(Error::Overflow)
        }
        let n: T = n.try_into().ok().unwrap();
        Ok(n)
    }

    fn to_unsigned<T>(self) -> Result<T>
    where
        T: Unsigned {
        
    }
}