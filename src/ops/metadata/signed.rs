macro_rules! impl_for {
    ($($ty:ident)*) => {
        $(
            impl Signed for $ty {}
        )*
    };
}

pub trait Signed {}

impl_for!(
    i8
    i16
    i32
    i64
    i128
    isize
);