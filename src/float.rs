pub trait Float
where
    Self: Sized,
    Self: Clone,
    Self: Copy,
    Self: Default,
    Self: PartialEq,
    Self: PartialOrd,
    Self: ::core::ops::Add<Output = Self>,
    Self: ::core::ops::AddAssign,
    Self: ::core::ops::Sub<Output = Self>,
    Self: ::core::ops::SubAssign
{}

impl Float for f32 {}
impl Float for f64 {}