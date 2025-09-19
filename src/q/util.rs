use super::*;

pub fn op<A, B>(on_op: A) -> Result<B>
where
    A: FnOnce() -> Result<B> {
    on_op()
}