pub trait PrimFloatConversion
where
    Self: ::core::convert::TryFrom<f32>,
    Self: ::core::convert::TryFrom<f64>
{}

impl<T> PrimFloatConversion for T
where
    Self: ::core::convert::TryFrom<f32>,
    Self: ::core::convert::TryFrom<f64>
{}