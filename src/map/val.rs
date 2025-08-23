pub trait Val
where
    Self: ::core::fmt::Debug,
    Self: Default,
    Self: Clone,
    Self: Copy {}

impl<A> Val for A
where
    A: ::core::fmt::Debug,
    A: Default,
    A: Clone,
    A: Copy {}