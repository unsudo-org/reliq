use super::*;

pub trait FromPrim {
    fn from<T>(n: T) -> Self
    where
        T: Prim;
}

impl FromPrim for f32 {
    fn from<T>(n: T) -> Self
    where
        T: Prim {
        match T::VARIANT {
            PrimVariant::U8 => n.try_into().ok().unwrap(),
            
        }
    }
}