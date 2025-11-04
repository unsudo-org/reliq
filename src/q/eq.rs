use super::*;

impl<const A: u8, B> Eq for Q<A, B>
where
    B: ops::Int {}

impl<const A: u8, B> PartialEq<Q<A, B>> for Q<A, B>
where
    B: ops::Int {
    #[inline]
    fn eq(&self, other: &Q<A, B>) -> bool {
        self == other
    }
}

impl<const A: u8, B> PartialEq<B> for Q<A, B>
where
    B: ops::Int {
    #[inline]
    fn eq(&self, other: &B) -> bool {
        &self.0 == other
    }
}

impl<const A: u8, B> PartialEq<f32> for Q<A, B>
where
    B: ops::Int {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        &self.0.to_f32().unwrap_or_default() == other
    }
}

impl<const A: u8, B> PartialEq<f64> for Q<A, B>
where
    B: ops::Int {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        &self.0.to_f64().unwrap_or_default() == other
    }
}