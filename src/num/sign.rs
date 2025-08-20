macro_rules! impl_for_signed {
    ($($ty:ident)*) => {
        $(
            impl Sign for $ty {
                const SIGNED: bool = true;
            }
        )*
    };
}

macro_rules! impl_for_unsigned {
    ($($ty:ident)*) => {
        $(
            impl Sign for $ty {
                const SIGNED: bool = false;
            }
        )*
    };
}

pub trait Sign {
    const SIGNED: bool;
}

impl_for_unsigned!(
    u8
    u16
    u32
    u64
    u128
    usize
);

impl_for_signed!(
    i8
    i16
    i32
    i64
    i128
    isize
);