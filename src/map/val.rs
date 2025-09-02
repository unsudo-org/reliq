pub trait Val
where
    Self: Clone,
    Self: Copy {}

impl<T> Val for T
where
    T: Clone,
    T: Copy {}