macro_rules! impl_for {
    ($($ty:ident)*) => {
        $(
            impl Unsigned for $ty {}
        )*
    };
}

pub trait Unsigned {}

impl_for!(
    u8
    u16
    u32
    u64
    u128
    usize
);