use super::*;

/// # Brief
/// `Q` can be represented in several semantic contexts.
/// Each context has a `Mode`.
/// 
/// # Extending
/// `Q` can represent more kinds of `Mode`'s.
/// ```rs
/// pub struct FooMode;
/// 
/// impl q::Mode for FooMode {}
/// 
/// 
/// ```
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
impl Mode for RatioMode {}
impl Mode for DeltaMode {}
impl Mode for CardinalMode {}