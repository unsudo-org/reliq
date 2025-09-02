pub trait Key
where
    Self: Clone,
    Self: Copy,
    Self: Eq,
    Self: ::core::hash::Hash {}

impl<T> Key for T
where
    T: Clone,
    T: Copy,
    T: Eq,
    T: ::core::hash::Hash {}