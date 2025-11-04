use super::*;

::modwire::expose!(
    pub arithmetic
    pub clone
    pub from
    pub lerp
    pub muldiv
    pub result
    pub scale
    pub supported
);

pub type Precision = u8;

#[repr(transparent)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub struct Q<const A: Precision, B>(B);