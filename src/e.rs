#[inline]
pub fn require<A, B, C, D>(state: &mut A, on_transaction: B) -> ::core::result::Result<C, D>
where
    A: Clone,
    B: FnOnce(&mut A) -> ::core::result::Result<C, D> {
    let snapshot: A = state.clone();
    match on_transaction(state) {
        Ok(ret) => Ok(ret),
        Err(e) => {
            *state = snapshot;
            Err(e)
        }
    }
}

#[inline]
pub fn retry<A, B, C>(attempts: usize, mut on_attempt: A) -> ::core::result::Result<B, C> 
where
    A: FnMut(usize) -> ::core::result::Result<B, C> {
    let mut last_failure: Option<C> = None;
    for attempt in 0..=attempts {
        match on_attempt(attempt) {
            Ok(ret) => return Ok(ret),
            Err(e) => last_failure = Some(e)
        }
    }
    let last_failure: C = unsafe {
        last_failure.unwrap_unchecked()
    };
    Err(last_failure)
}