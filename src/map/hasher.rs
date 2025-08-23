pub trait Hasher
where
    Self: Default,
    Self: ::core::hash::Hasher {}

impl<A> Hasher for A
where
    A: Default,
    A: ::core::hash::Hasher {}