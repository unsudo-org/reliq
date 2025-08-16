pub trait Num
where
    Self: Sized,
    Self: ::core::fmt::Debug,
    Self: ::core::fmt::Display,
    Self: Clone,
    Self: Copy,
    Self: Default,
    Self: PartialEq,
    Self: PartialOrd,
    Self: ::core::ops::Add<Output = Self>,
    Self: ::core::ops::AddAssign,
    Self: ::core::ops::Sub<Output = Self>,
    Self: ::core::ops::SubAssign,
    Self: ::core::ops::Mul<Output = Self>,
    Self: ::core::ops::MulAssign,
    Self: ::core::ops::Div<Output = Self>,
    Self: ::core::ops::DivAssign,
    Self: ::core::ops::Rem<Output = Self>,
    Self: ::core::ops::RemAssign 
{}

impl<T> Num for T
where
    T: Sized,
    T: ::core::fmt::Debug,
    T: ::core::fmt::Display,
    T: Clone,
    T: Copy,
    T: Default,
    T: PartialEq,
    T: PartialOrd,
    T: ::core::ops::Add<Output = Self>,
    T: ::core::ops::AddAssign,
    T: ::core::ops::Sub<Output = Self>,
    T: ::core::ops::SubAssign,
    T: ::core::ops::Mul<Output = Self>,
    T: ::core::ops::MulAssign,
    T: ::core::ops::Div<Output = Self>,
    T: ::core::ops::DivAssign,
    T: ::core::ops::Rem<Output = Self>,
    T: ::core::ops::RemAssign
{}