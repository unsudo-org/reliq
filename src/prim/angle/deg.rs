use super::*;

#[repr(transparent)]
pub struct Deg<const A: u8, B, C>(q::Q<A, B, C>)
where
    B: num::Int,
    B: num::Prim,
    C: q::Engine,
    (): q::Precision<A>,
    (): q::N<B>,
    (): q::ScaleCompatible<A, B>,
    (): q::PICompatible<A, B>;

impl<const A: u8, B, C> TryFrom<rad::Rad<A, B, C>> for Deg<A, B, C> 
where
    B: num::Int,
    B: num::Prim,
    C: q::Engine,
    (): q::Precision<A>,
    (): q::N<B>,
    (): q::ScaleCompatible<A, B>,
    (): q::PICompatible<A, B> {
    type Error = Error;

    fn try_from(value: rad::Rad<A, B, C>) -> ::core::result::Result<Self, Self::Error> {
        
    }
}

impl<const A: u8, B, C> Angle for Deg<A, B, C> 
where
    B: num::Int,
    B: num::Prim,
    C: q::Engine,
    (): q::Precision<A>,
    (): q::N<B>,
    (): q::ScaleCompatible<A, B>,
    (): q::PICompatible<A, B> {
    
}