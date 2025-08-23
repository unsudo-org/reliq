use super::*;

pub struct Iter<const A: usize, B> {
    pub(super) buf: [::core::mem::MaybeUninit<B>; A],
    pub(super) len: usize,
    pub(super) key: usize
}

impl<const A: usize, B> Iterator for Iter<A, B> {
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.key >= self.len {
            return None
        }
        let data = unsafe {
            self.buf[self.key].assume_init_read()
        };
        self.key += 1;
        Some(data)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let left: usize = self.len - self.key;
        (left, Some(left))
    }
}