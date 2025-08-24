use super::*;

pub trait Key
where
    Self: ink::Ink,
    Self: ::core::fmt::Debug,
    Self: Default,
    Self: Clone,
    Self: Copy,
    Self: Eq,
    Self: ::core::hash::Hash {}

impl<T> Key for T
where
    T: ink::Ink,
    T: ::core::fmt::Debug,
    T: Default,
    T: Clone,
    T: Copy,
    T: Eq,
    T: ::core::hash::Hash {}