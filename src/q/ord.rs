use super::*;

impl<const A: u8, B, C> Ord for Q<A, B, C>
where
    B: ops::Int,
    C: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self
    where
        Self: Sized {
        if self > other {
            self
        } else {
            other
        }
    }

    #[inline]
    fn min(self, other: Self) -> Self
    where
        Self: Sized {
        if self < other {
            self
        } else {
            other
        }
    }

    #[inline]
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        let x: B = self.n;
        let y: B = other.n;
        if x > y {
            ::core::cmp::Ordering::Greater
        } else if x < y {
            ::core::cmp::Ordering::Less
        } else {
            ::core::cmp::Ordering::Equal
        }
    }
}

impl<const A: u8, B, C, D> PartialOrd<Q<A, B, C>> for Q<A, B, D>
where
    B: ops::Int,
    C: Mode,
    D: Mode,
    (): SupportedPrecision<A>,
    (): SupportedInt<B>,
    (): Supported<A, B> {
    
    #[inline]
    fn ge(&self, other: &Q<A, B, C>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x >= y
    }

    #[inline]
    fn le(&self, other: &Q<A, B, C>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x <= y
    }

    #[inline]
    fn gt(&self, other: &Q<A, B, C>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x > y
    }

    #[inline]
    fn lt(&self, other: &Q<A, B, C>) -> bool {
        let x: B = self.n;
        let y: B = other.n;
        x < y
    }
    
    #[allow(clippy::non_canonical_partial_ord_impl)]
    #[inline]
    fn partial_cmp(&self, other: &Q<A, B, C>) -> Option<core::cmp::Ordering> {
        let x: B = self.n;
        let y: B = other.n;
        let ret: ::core::cmp::Ordering = x.cmp(&y);
        Some(ret)
    }
}