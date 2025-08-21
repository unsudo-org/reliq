use super::*;

pub trait Mode 
where
    Self: Sized,
    Self: Clone,
    Self: Copy {}

impl Mode for DefaultMode {}
impl Mode for RadMode {}
impl Mode for DegMode {}