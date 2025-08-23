pub trait Key
where
    Self: ::core::fmt::Debug,
    Self: Default,
    Self: Clone,
    Self: Copy,
    Self: Eq,
    Self: ::core::hash::Hash {}

impl<A> Key for A
where
    A: ::core::fmt::Debug,
    A: Default,
    A: Clone,
    A: Copy,
    A: Eq,
    A: ::core::hash::Hash {}