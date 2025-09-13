use super::*;

pub struct ConstBuilder<const A: usize, B> 
where
    B: Clone,
    B: Copy {
    children: Array<A, B>,
    outcome: Option<Result<()>>
}