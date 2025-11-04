use super::*;

impl<const A: u8, B> Q<A, B>
where
    B: ops::Int {
    #[inline]
    pub fn to_negative(self) -> Self 
    where
        B: ops::Signed {
        let n: B = self.0;
        if n == B::AS_0 {
            return self
        }
        let n: B = B::AS_0 - n;
        let ret: Self = n.into();
        ret
    }

    #[inline]
    pub fn to_positive(self) -> Self 
    where
        B: ops::Signed {
        let n: B = self.0;
        if n >= B::AS_0 {
            return self
        }
        let n: B = B::AS_0 - n;
        let ret: Self = n.into();
        ret
    }
}