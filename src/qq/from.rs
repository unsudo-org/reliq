use super::*;

impl<const A: Precision, B> From<B> for Q<A, B> {
    fn from(value: B) -> Self {
        Self(value)
    }
}