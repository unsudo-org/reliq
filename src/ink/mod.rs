#[cfg(feature = "ink")]
pub trait Ink 
where
    Self: ::scale::Encode,
    Self: ::scale::Decode,
    Self: ::scale_info::TypeInfo {}

#[cfg(not(feature = "ink"))]
pub trait Ink {}

#[cfg(not(feature = "ink"))]
impl<T> Ink for T {}