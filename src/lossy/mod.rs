/// A wrapper to indicate that the operation may
/// result in partial loss of data ie precision
/// related information or color data which
/// cannot be recovered.
pub struct Lossy<T>(T);

impl<A> Lossy<A> {
    pub fn new(x: A) -> Self {
        Self(x)
    }

    pub fn allow_lossy(self) -> A {
        self.0
    }

    pub fn map<B, C>(self, on_map: C) -> Lossy<B>
    where
        C: FnOnce(A) -> B {
        Lossy(on_map(self.0))
    }
}
