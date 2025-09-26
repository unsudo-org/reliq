use super::*;
use ops::ToPrim as _;

::modwire::expose!(
    pub arithmetic
    pub r#as
    pub cast
    pub deg
    pub delta
    pub engine
    pub eq
    pub error
    pub factor
    pub fmt
    pub from
    pub lerp
    pub mode
    pub muldiv
    pub ord
    pub percentage
    pub pi
    pub rad
    pub ratio
    pub round
    pub scale
    pub sign
    pub sqrt
    pub supported
    pub to_prim
    pub unit
    pub util
);

#[macro_export(local_inner_macros)]
macro_rules! mode {
    (
        $(#[$attr:meta])*
        $mode:ident
    ) => {
        ::paste::paste!(
            $(#[$attr])*
            pub type $mode<const A: u8, B = usize> = Q<A, B, [< $mode Mode >]>;
            
            $(#[$attr])*
            pub type [< $mode 1 >]<T = usize> = $mode<1, T>;

            $(#[$attr])*
            pub type [< $mode 2 >]<T = usize> = $mode<2, T>;

            $(#[$attr])*
            pub type [< $mode 3 >]<T = usize> = $mode<3, T>;

            $(#[$attr])*
            pub type [< $mode 4 >]<T = usize> = $mode<4, T>;

            $(#[$attr])*
            pub type [< $mode 5 >]<T = usize> = $mode<5, T>;

            $(#[$attr])*
            pub type [< $mode 6 >]<T = usize> = $mode<6, T>;

            $(#[$attr])*
            pub type [< $mode 7 >]<T = usize> = $mode<7, T>;

            $(#[$attr])*
            pub type [< $mode 8 >]<T = usize> = $mode<8, T>;

            $(#[$attr])*
            pub type [< $mode 9 >]<T = usize> = $mode<9, T>;

            $(#[$attr])*
            pub type [< $mode 10 >]<T = usize> = $mode<10, T>;

            $(#[$attr])*
            pub type [< $mode 11 >]<T = usize> = $mode<11, T>;

            $(#[$attr])*
            pub type [< $mode 12 >]<T = usize> = $mode<12, T>;

            $(#[$attr])*
            pub type [< $mode 13 >]<T = usize> = $mode<13, T>;

            $(#[$attr])*
            pub type [< $mode 14 >]<T = usize> = $mode<14, T>;

            $(#[$attr])*
            pub type [< $mode 15 >]<T = usize> = $mode<15, T>;
        
            $(#[$attr])*
            pub type [< $mode 16 >]<T = usize> = $mode<16, T>;

            $(#[$attr])*
            pub type [< $mode 17 >]<T = usize> = $mode<17, T>;

            $(#[$attr])*
            pub type [< $mode 18 >]<T = usize> = $mode<18, T>;

            $(#[$attr])*
            pub type [< $mode 19 >]<T = usize> = $mode<19, T>;

            $(#[$attr])*
            pub type [< $mode 20 >]<T = usize> = $mode<20, T>;

            $(#[$attr])*
            pub type [< $mode 21 >]<T = usize> = $mode<21, T>;

            $(#[$attr])*
            pub type [< $mode 22 >]<T = usize> = $mode<22, T>;

            $(#[$attr])*
            pub type [< $mode 23 >]<T = usize> = $mode<23, T>;

            $(#[$attr])*
            pub type [< $mode 24 >]<T = usize> = $mode<24, T>;

            $(#[$attr])*
            pub type [< $mode 25 >]<T = usize> = $mode<25, T>;

            $(#[$attr])*
            pub type [< $mode 26 >]<T = usize> = $mode<26, T>;

            $(#[$attr])*
            pub type [< $mode 27 >]<T = usize> = $mode<27, T>;

            $(#[$attr])*
            pub type [< $mode 28 >]<T = usize> = $mode<28, T>;

            $(#[$attr])*
            pub type [< $mode 29 >]<T = usize> = $mode<29, T>;

            $(#[$attr])*
            pub type [< $mode 30 >]<T = usize> = $mode<30, T>;

            $(#[$attr])*
            pub type [< $mode 31 >]<T = usize> = $mode<31, T>;

            $(#[$attr])*
            pub type [< $mode 32 >]<T = usize> = $mode<32, T>;

            $(#[$attr])*
            pub type [< $mode 33 >]<T = usize> = $mode<33, T>;

            $(#[$attr])*
            pub type [< $mode 34 >]<T = usize> = $mode<34, T>;

            $(#[$attr])*
            pub type [< $mode 35 >]<T = usize> = $mode<35, T>;

            $(#[$attr])*
            pub type [< $mode 36 >]<T = usize> = $mode<36, T>;

            $(#[$attr])*
            pub type [< $mode 37 >]<T = usize> = $mode<37, T>;

            #[derive(Debug)]
            #[derive(Clone)]
            #[derive(Copy)]
            pub struct [< $mode Mode >];

            impl Mode for [< $mode Mode >] {}
        );
    };
}

#[repr(transparent)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub struct Q<const A: u8 = 2, B = usize, C = UnitMode>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    n: B,
    m_0: ::core::marker::PhantomData<C>
}

impl<const A: u8, B, C> Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    pub fn from_raw(n: B) -> Self {
        Self {
            n,
            m_0: ::core::marker::PhantomData
        }
    }



    #[inline]
    pub fn into_int(self) -> B {
        self.n
    }
}