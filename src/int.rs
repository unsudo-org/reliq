macro_rules! const_as {
    ($($n:literal)*) => {
        paste::paste!(
            $(
                const [< AS _ $n >]: Self;
            )*
        );
    };
}

macro_rules! const_as_impl {
    ($($n:literal)*) => {
        paste::paste!(
            $(
                const [< AS_ $n >]: Self = $n;
            )*
        );
    };
}

macro_rules! common_impl {
    () => {
        const_as_impl!(
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

macro_rules! signed_impl {
    ($ty:ident) => {
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
            common_impl!();
        }
    };
    ($($ty:ident)*) => {
        $(
            signed_impl!($ty);
        )*
    };
}

macro_rules! unsigned_impl {
    ($ty:ident) => {
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
            common_impl!();
        }
    };
    ($($ty:ident)*) => {
        $(
            unsigned_impl!($ty);
        )*
    };
}

pub trait Int 
where
    Self: Sized,
    Self: ::core::fmt::Debug,
    Self: ::core::fmt::Display,
    Self: Default,
    Self: Clone,
    Self: Copy,
    Self: PartialEq,
    Self: Eq,
    Self: PartialOrd,
    Self: Ord,
    Self: TryFrom<i8>,
    Self: TryFrom<i16>,
    Self: TryFrom<i32>,
    Self: TryFrom<i64>,
    Self: TryFrom<i128>,
    Self: TryFrom<isize>,
    Self: TryFrom<u8>,
    Self: TryFrom<u16>,
    Self: TryFrom<u32>,
    Self: TryFrom<u64>,
    Self: TryFrom<u128>,
    Self: TryFrom<usize>,
    Self: TryInto<i8>,
    Self: TryInto<i16>,
    Self: TryInto<i32>,
    Self: TryInto<i64>,
    Self: TryInto<i128>,
    Self: TryInto<isize>,
    Self: TryInto<u8>,
    Self: TryInto<u16>,
    Self: TryInto<u32>,
    Self: TryInto<u64>,
    Self: TryInto<u128>,
    Self: TryInto<usize>,
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
    Self: ::core::ops::Shl<Output = Self>,
    Self: ::core::ops::ShlAssign,
    Self: ::core::ops::Shr<Output = Self>,
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
    const_as!(
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

signed_impl!(
    i8
    i16
    i32
    i64
    i128
    isize
);

unsigned_impl!(
    u8 
    u16 
    u32 
    u64 
    u128
    usize
);