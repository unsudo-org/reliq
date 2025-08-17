use super::*;

pub trait Mode {}
impl Mode for DefaultMode {}
impl Mode for RatioMode {}
impl Mode for PercentageMode {}
impl Mode for ChanceMode {}
impl Mode for RadMode {}
impl Mode for DegMode {}