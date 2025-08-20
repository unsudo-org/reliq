macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl Prim for $ty {}
        )*
    };
}

pub trait Prim {}

r#impl!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
);