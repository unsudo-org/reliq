//! Num

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

macro_rules! common_int_impl_block_fragment {
    () => {
        fn checked_add(self, rhs: Self) -> Option<Self> {
            self.checked_add(rhs)
        }

        fn checked_sub(self, rhs: Self) -> Option<Self> {
            self.checked_sub(rhs)
        }

        fn checked_mul(self, rhs: Self) -> Option<Self> {
            self.checked_mul(rhs)
        }

        fn checked_div(self, rhs: Self) -> Option<Self> {
            self.checked_div(rhs)
        }

        fn checked_rem(self, rhs: Self) -> Option<Self> {
            self.checked_rem(rhs)
        }

        fn checked_shl(self, rhs: u32) -> Option<Self> {
            self.checked_shl(rhs)
        }

        fn checked_shr(self, rhs: u32) -> Option<Self> {
            self.checked_shr(rhs)
        }

        fn checked_neg(self) -> Option<Self> {
            self.checked_neg()
        }

        fn wrapping_add(self, rhs: Self) -> Self {
            self.wrapping_add(rhs)
        }

        fn wrapping_sub(self, rhs: Self) -> Self {
            self.wrapping_sub(rhs)
        }

        fn wrapping_mul(self, rhs: Self) -> Self {
            self.wrapping_mul(rhs)
        }

        fn wrapping_div(self, rhs: Self) -> Self {
            self.wrapping_div(rhs)
        }

        fn wrapping_rem(self, rhs: Self) -> Self {
            self.wrapping_rem(rhs)
        }

        fn wrapping_pow(self, exponent: u32) -> Self {
            self.wrapping_pow(exponent)
        }

        fn wrapping_neg(self) -> Self {
            self.wrapping_neg()
        }

        fn overflowing_add(self, rhs: Self) -> (Self, bool) {
            self.overflowing_add(rhs)
        }

        fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
            self.overflowing_sub(rhs)
        }

        fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
            self.overflowing_mul(rhs)
        }

        fn overflowing_div(self, rhs: Self) -> (Self, bool) {
            self.overflowing_div(rhs)
        }

        fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
            self.overflowing_rem(rhs)
        }

        fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
            self.overflowing_shl(rhs)
        }

        fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
            self.overflowing_shr(rhs)
        }

        fn overflowing_neg(self) -> (Self, bool) {
            self.overflowing_neg()
        }

        fn sqrt(self) -> Self {
            self.isqrt()
        }
    };
}

macro_rules! common_signed_int_impl {
    ($($ty:ident)*) => {
        $(
            impl Int for $ty {
                const SIGNED: bool = true;
                const BITS: Self = $ty::BITS as Self;
                const BITS_U128: u128 = Self::BITS as u128;
                const BITS_I128: i128 = Self::BITS as i128;
                const MAX: Self = $ty::MAX as Self;
                const MAX_U128: u128 = Self::MAX as u128;
                const MAX_I128: i128 = Self::MAX as i128;
                const MIN: Self = $ty::MIN as Self;
                const MIN_U128: u128 = Self::MIN as u128;
                const MIN_I128: i128 = Self::MIN as i128;
                common_int_impl_block_fragment!();
            }
        )*
    };
}

macro_rules! common_unsigned_int_impl {
    ($($ty:ident)*) => {
        $(
            impl Int for $ty {
                const SIGNED: bool = false;
                const BITS: Self = $ty::BITS as Self;
                const BITS_U128: u128 = Self::BITS as u128;
                const BITS_I128: i128 = Self::BITS as i128;
                const MAX: Self = $ty::MAX as Self;
                const MAX_U128: u128 = Self::MAX as u128;
                const MAX_I128: i128 = Self::MAX as i128;
                const MIN: Self = $ty::MIN as Self;
                const MIN_U128: u128 = Self::MIN as u128;
                const MIN_I128: i128 = Self::MIN as i128;
                common_int_impl_block_fragment!();
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


pub trait Int 
where
    Self: Num,
    Self: ::core::cmp::Eq,
    Self: ::core::cmp::Ord,
    Self: ::core::convert::TryFrom<i8>,
    Self: ::core::convert::TryFrom<i16>,
    Self: ::core::convert::TryFrom<i32>,
    Self: ::core::convert::TryFrom<i64>,
    Self: ::core::convert::TryFrom<i128>,
    Self: ::core::convert::TryFrom<isize>,
    Self: ::core::convert::TryFrom<u8>,
    Self: ::core::convert::TryFrom<u16>,
    Self: ::core::convert::TryFrom<u32>,
    Self: ::core::convert::TryFrom<u64>,
    Self: ::core::convert::TryFrom<u128>,
    Self: ::core::convert::TryFrom<usize>,
    Self: ::core::convert::TryInto<i8>,
    Self: ::core::convert::TryInto<i16>,
    Self: ::core::convert::TryInto<i32>,
    Self: ::core::convert::TryInto<i64>,
    Self: ::core::convert::TryInto<i128>,
    Self: ::core::convert::TryInto<isize>,
    Self: ::core::convert::TryInto<u8>,
    Self: ::core::convert::TryInto<u16>,
    Self: ::core::convert::TryInto<u32>,
    Self: ::core::convert::TryInto<u64>,
    Self: ::core::convert::TryInto<u128>,
    Self: ::core::convert::TryInto<usize>,
    Self: ::core::ops::Shl<u32, Output = Self>,
    Self: ::core::ops::ShlAssign,
    Self: ::core::ops::Shr<u32, Output = Self>,
    Self: ::core::ops::ShrAssign,
    Self: ::core::ops::BitAnd<Output = Self>,
    Self: ::core::ops::BitAndAssign,
    Self: ::core::ops::BitOr<Output = Self>,
    Self: ::core::ops::BitOrAssign,
    Self: ::core::ops::BitXor<Output = Self>,
    Self: ::core::ops::BitXorAssign {
    const SIGNED: bool;
    const BITS: Self;
    const BITS_U128: u128;
    const BITS_I128: i128;
    const MAX: Self;
    const MAX_U128: u128;
    const MAX_I128: i128;
    const MIN: Self;
    const MIN_U128: u128 = 0;
    const MIN_I128: i128 = 0;
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;
    fn checked_shl(self, rhs: u32) -> Option<Self>;
    fn checked_shr(self, rhs: u32) -> Option<Self>;
    fn checked_neg(self) -> Option<Self>;
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn wrapping_rem(self, rhs: Self) -> Self;
    fn wrapping_pow(self, exponent: u32) -> Self;
    fn wrapping_neg(self) -> Self;
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);
    fn overflowing_shl(self, rhs: u32) -> (Self, bool);
    fn overflowing_shr(self, rhs: u32) -> (Self, bool);
    fn overflowing_neg(self) -> (Self, bool);
    fn sqrt(self) -> Self;
}

common_signed_int_impl!(
    i8
    i16
    i32
    i64
    i128
    isize
);

common_unsigned_int_impl!(
    u8
    u16
    u32
    u64
    u128
    usize
);


pub trait Float
where
    Self: Sized,
    Self: Num
{}

impl Float for f32 {}
impl Float for f64 {}


pub trait Prim 
where
    Self: Sized,
    Self: Copy{}
impl Prim for u8 {}
impl Prim for u16 {}
impl Prim for u32 {}
impl Prim for u64 {}
impl Prim for u128 {}
impl Prim for usize {}
impl Prim for i8 {}
impl Prim for i16 {}
impl Prim for i32 {}
impl Prim for i64 {}
impl Prim for i128 {}
impl Prim for isize {}
impl Prim for f32 {}
impl Prim for f64 {}