macro_rules! common_const {
    ($($n:literal)*) => {
        ::paste::paste!(
            $(
                const [< AS _ $n >]: Self;
            )*
        );
    };
}

macro_rules! common_const_impl {
    ($ty:ident $($n:literal)*) => {
        ::paste::paste!(
            $(
                const [< AS _ $n >]: Self = [< $n $ty >];
            )*
        );
    };
}

macro_rules! common_num_float_impl {
    ($($ty:ident)*) => {
        $(
            impl Num for $ty {}
            impl MaybeBound for $ty {}
            
            impl ConstAs for $ty {
                common_const_impl!(
                    $ty
                    0 1 2 3 4 5 6 7 8 9
                    10 11 12 13 14 15 16 17 18 19 
                    20 21 22 23 24 25 26 27 28 29 
                    30 31 32 33 34 35 36 37 38 39
                    40 41 42 43 44 45 46 47 48 49
                    50 51 52 53 54 55 56 57 58 59
                    60 61 62 63 64 65 66 67 68 69
                    70 71 72 73 74 75 76 77 78 79
                    80 81 82 83 84 85 86 87 88 89
                    90 91 92 93 94 95 96 97 98 99
                    100
                );
            }
        )*
    };
}

macro_rules! common_num_int_impl {
    ($($ty:ident)*) => {
        $(
            impl Num for $ty {}
            
            impl MaybeBound for $ty {
                const MIN: Option<Self> = Some($ty::MIN);
                const MIN_U128: Option<u128> = Some($ty::MIN as u128);
                const MIN_I128: Option<i128> = Some($ty::MIN as i128);
                const MAX: Option<Self> = Some($ty::MAX);
                const MAX_U128: Option<u128> = Some($ty::MAX as u128);
                const MAX_I128: Option<i128> = Some($ty::MAX as i128);
            }

            impl ConstAs for $ty {
                common_const_impl!(
                    $ty
                    0 1 2 3 4 5 6 7 8 9
                    10 11 12 13 14 15 16 17 18 19 
                    20 21 22 23 24 25 26 27 28 29 
                    30 31 32 33 34 35 36 37 38 39
                    40 41 42 43 44 45 46 47 48 49
                    50 51 52 53 54 55 56 57 58 59
                    60 61 62 63 64 65 66 67 68 69
                    70 71 72 73 74 75 76 77 78 79
                    80 81 82 83 84 85 86 87 88 89
                    90 91 92 93 94 95 96 97 98 99
                    100
                );
            }
        )*
    };
}

pub trait ConstAs {
    common_const!(
        0 1 2 3 4 5 6 7 8 9
        10 11 12 13 14 15 16 17 18 19 
        20 21 22 23 24 25 26 27 28 29 
        30 31 32 33 34 35 36 37 38 39
        40 41 42 43 44 45 46 47 48 49
        50 51 52 53 54 55 56 57 58 59
        60 61 62 63 64 65 66 67 68 69
        70 71 72 73 74 75 76 77 78 79
        80 81 82 83 84 85 86 87 88 89
        90 91 92 93 94 95 96 97 98 99
        100
    );
}

pub trait MaybeBound 
where
    Self: Sized {
    const MIN: Option<Self> = None;
    const MIN_U128: Option<u128> = None;
    const MIN_I128: Option<i128> = None;
    const MAX: Option<Self> = None;
    const MAX_U128: Option<u128> = None;
    const MAX_I128: Option<i128> = None;
}

pub trait Num
where
    Self: ConstAs,
    Self: MaybeBound,
    Self: ::core::default::Default,
    Self: ::core::clone::Clone,
    Self: ::core::marker::Sized,
    Self: ::core::marker::Copy,
    Self: ::core::cmp::PartialEq,
    Self: ::core::cmp::PartialOrd,
    Self: ::core::fmt::Debug,
    Self: ::core::fmt::Display,
    Self: ::core::ops::Add<Output = Self>,
    Self: ::core::ops::AddAssign,
    Self: ::core::ops::Sub<Output = Self>,
    Self: ::core::ops::SubAssign,
    Self: ::core::ops::Mul<Output = Self>,
    Self: ::core::ops::MulAssign,
    Self: ::core::ops::Div<Output = Self>,
    Self: ::core::ops::DivAssign,
    Self: ::core::ops::Rem<Output = Self>,
    Self: ::core::ops::RemAssign,
    Self: for<'a> ::core::ops::Add<&'a Self, Output = Self>,
    Self: for<'a> ::core::ops::AddAssign<&'a Self>,
    Self: for<'a> ::core::ops::Sub<&'a Self, Output = Self>,
    Self: for<'a> ::core::ops::SubAssign<&'a Self>,
    Self: for<'a> ::core::ops::Mul<&'a Self, Output = Self>,
    Self: for<'a> ::core::ops::MulAssign<&'a Self>,
    Self: for<'a> ::core::ops::Div<&'a Self, Output = Self>,
    Self: for<'a> ::core::ops::DivAssign<&'a Self>,
    Self: for<'a> ::core::ops::Rem<&'a Self, Output = Self>,
    Self: for<'a> ::core::ops::RemAssign<&'a Self>
{}

common_num_float_impl!(
    f32
    f64
);

common_num_int_impl!(
    i8
    i16
    i32
    i64
    i128
    isize
    u8
    u16
    u32
    u64
    u128
    usize
);