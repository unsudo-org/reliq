use super::*;

pub struct Point2D<const A: u8, B, C> 
where
    B: num::Int,
    B: num::Prim,
    C: q::Engine,
    (): q::Precision<A>,
    (): q::N<B>,
    (): q::ScaleCompatible<A, B>,
    (): q::PICompatible<A, B> {
    pub x: q::Q<A, B, C>,
    pub y: q::Q<A, B, C>
}

impl<const A: u8, B, C> From<(B, B)> for Point2D<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: q::Engine,
    (): q::Precision<A>,
    (): q::N<B>,
    (): q::ScaleCompatible<A, B>,
    (): q::PICompatible<A, B> {
    #[inline]
    fn from(value: (B, B)) -> Self {
        let x: q::Q<A, B, C> = value.0.into();
        let y: q::Q<A, B, C> = value.1.into();
        Self {
            x,
            y
        }
    }
}

impl<const A: u8, B, C> From<(q::Q<A, B, C>, q::Q<A, B, C>)> for Point2D<A, B, C>
where
    B: num::Int,
    B: num::Prim,
    C: q::Engine,
    (): q::Precision<A>,
    (): q::N<B>,
    (): q::ScaleCompatible<A, B>,
    (): q::PICompatible<A, B> {
    #[inline]
    fn from(value: (q::Q<A, B, C>, q::Q<A, B, C>)) -> Self {
        let x: q::Q<A, B, C> = value.0;
        let y: q::Q<A, B, C> = value.1;
        Self {
            x,
            y
        }
    }
}

impl<const A: u8, B, C> Coordinate<A, B, C> for Point2D<A, B, C> 
where
    B: num::Int,
    B: num::Prim,
    C: q::Engine,
    (): q::Precision<A>,
    (): q::N<B>,
    (): q::ScaleCompatible<A, B>,
    (): q::PICompatible<A, B> {
    #[inline]
    fn distance_between(self, rhs: Self) -> q::Q<A, B, C> {
        
    }
}