use super::*;

pub trait Mode 
where
    Self: Sized,
    Self: Clone,
    Self: Copy {}

impl Mode for ChanceMode {}
impl Mode for UnitMode {}
impl Mode for DegMode {}
impl Mode for RadMode {}
impl Mode for PercentageMode {}
impl Mode for FactorMode {}