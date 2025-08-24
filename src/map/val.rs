pub trait Val
where
    Self: ::core::fmt::Debug,
    Self: Default,
    Self: Clone,
    Self: Copy {}

impl<T> Val for T
where
    T: ::core::fmt::Debug,
    T: Default,
    T: Clone,
    T: Copy {}