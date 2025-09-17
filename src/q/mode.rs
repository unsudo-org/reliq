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