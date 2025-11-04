use super::*;

::modwire::expose!(
    pub arithmetic
    pub r#as
    pub cast
    pub clone
    pub eq
    pub from
    pub lerp
    pub muldiv
    pub ord
    pub pi
    pub result
    pub round
    pub scale
    pub sign
    pub sqrt
    pub supported
    pub to_prim
    pub trig
);

pub type Precision = u8;

#[repr(transparent)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub struct Q<const A: Precision, B>(B);