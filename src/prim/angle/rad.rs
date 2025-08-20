use super::*;

pub struct Rad<const A: u8, B, C>(q::Q<A, B, C>)
where
    B: num::Int,
    B: num::Prim,
    C: q::Engine,
    (): q::Precision<A>,
    (): q::N<B>,
    (): q::ScaleCompatible<A, B>,
    (): q::PICompatible<A, B>;