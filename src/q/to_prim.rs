use super::*;

impl<const A: u8, B, C> ops::ToPrim for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn to_u8(&self) -> ops::Result<u8> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u8 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_u16(&self) -> ops::Result<u16> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u16 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_u32(&self) -> ops::Result<u32> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u32 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_u64(&self) -> ops::Result<u64> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u64 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_u128(&self) -> ops::Result<u128> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: u128 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)   
    }

    #[inline]
    fn to_usize(&self) -> ops::Result<usize> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: usize = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_i8(&self) -> ops::Result<i8> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i8 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_i16(&self) -> ops::Result<i16> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i16 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_i32(&self) -> ops::Result<i32> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i32 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)   
    }

    #[inline]
    fn to_i64(&self) -> ops::Result<i64> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i64 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_i128(&self) -> ops::Result<i128> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: i128 = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)
    }

    #[inline]
    fn to_isize(&self) -> ops::Result<isize> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: isize = ret
            .try_into()
            .ok()
            .ok_or(ops::Error::UnsupportedConversion)?;
        Ok(ret)   
    }

    #[inline]
    fn to_f32(&self) -> ops::Result<f32> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: f32 = ret.to_f32()?;
        Ok(ret)
    }

    #[inline]
    fn to_f64(&self) -> ops::Result<f64> {
        let ret: B = scale::<A, B>();
        let ret: B = self.n / ret;
        let ret: f64 = ret.to_f64()?;
        Ok(ret)
    }
}