use super::*;

pub trait Val
where
    Self: ink::Ink,
    Self: ::core::fmt::Debug,
    Self: Default,
    Self: Clone,
    Self: Copy {}

impl<T> Val for T
where
    T: ink::Ink,
    T: ::core::fmt::Debug,
    T: Default,
    T: Clone,
    T: Copy {}