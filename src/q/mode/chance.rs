use super::*;

#[repr(transparent)]
pub struct ChanceMode;

impl Mode for ChanceMode {}

impl<const A: u8, B, C> Q<A, B, ChanceMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    pub fn roll<const D: usize>(self, seed: [u8; D]) -> bool {

    }
}

impl<const A: u8, B, C> TryFrom<B> for Q<A, B, ChanceMode, C>
where
    B: num::Int,
    C: Engine,
    (): Precision<A>,
    (): N<B> {
    type Error = Error;
    
    fn try_from(n: B) -> ::core::result::Result<Self, Self::Error> {
        if n < B::AS_0 {
            return Err()
        }
        if n > B::AS_1 {
            return Err()
        }
        Ok(Q::new(n))
    }
}