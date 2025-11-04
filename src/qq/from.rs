use super::*;

impl<const A: Precision, B> From<B> for Q<A, B> {
    #[inline]
    fn from(value: B) -> Self {
        Self(value)
    }
}