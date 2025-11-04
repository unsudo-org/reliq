use super::*;

impl<const A: Precision, B> Clone for Q<A, B> 
where
    B: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.to_owned())
    }
}

impl<const A: Precision, B> Copy for Q<A, B> 
where
    B: Copy {}