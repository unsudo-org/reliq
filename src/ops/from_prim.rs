use super::*;

macro_rules! r#impl {
    ($($ty:ty)*) => {
        ::paste::paste!(
            $(
                impl FromPrim for $ty {
                    fn from<T>(n: T) -> Result<Self>
                    where
                        T: Prim,
                        T: ToPrim {
                        match T::VARIANT {
                            PrimVariant::U8 => n.[< to_ $ty >](),
                            PrimVariant::U16 => n.[< to_ $ty >](),
                            PrimVariant::U32 => n.[< to_ $ty >](),
                            PrimVariant::U64 => n.[< to_ $ty >](),
                            PrimVariant::U128 => n.[< to_ $ty >](),
                            PrimVariant::USize => n.[< to_ $ty >](),
                            PrimVariant::I8 => n.[< to_ $ty >](),
                            PrimVariant::I16 => n.[< to_ $ty >](),
                            PrimVariant::I32 => n.[< to_ $ty >](),
                            PrimVariant::I64 => n.[< to_ $ty >](),
                            PrimVariant::I128 => n.[< to_ $ty >](),
                            PrimVariant::ISize => n.[< to_ $ty >](),
                            PrimVariant::F32 => n.[< to_ $ty >](),
                            PrimVariant::F64 => n.[< to_ $ty >]()
                        }
                    }
                }
            )*
        );
    };
}

pub trait FromPrim 
where
    Self: Sized {
    fn from<T>(n: T) -> Result<Self>
    where
        T: Prim,
        T: ToPrim;
}

r#impl!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
);