use super::*;

impl<const A: u8, B> Ord for Q<A, B>
where
    B: ops::Int {
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
        let x: B = self.0;
        let y: B = other.0;
        if x > y {
            ::core::cmp::Ordering::Greater
        } else if x < y {
            ::core::cmp::Ordering::Less
        } else {
            ::core::cmp::Ordering::Equal
        }
    }
}

impl<const A: u8, B> PartialOrd<Q<A, B>> for Q<A, B>
where
    B: ops::Int {
    #[inline]
    fn ge(&self, other: &Q<A, B>) -> bool {
        let x: B = self.0;
        let y: B = other.0;
        x >= y
    }

    #[inline]
    fn le(&self, other: &Q<A, B>) -> bool {
        let x: B = self.0;
        let y: B = other.0;
        x <= y
    }

    #[inline]
    fn gt(&self, other: &Q<A, B>) -> bool {
        let x: B = self.0;
        let y: B = other.0;
        x > y
    }

    #[inline]
    fn lt(&self, other: &Q<A, B>) -> bool {
        let x: B = self.0;
        let y: B = other.0;
        x < y
    }
    
    #[allow(clippy::non_canonical_partial_ord_impl)]
    #[inline]
    fn partial_cmp(&self, other: &Q<A, B>) -> Option<::core::cmp::Ordering> {
        let x: B = self.0;
        let y: B = other.0;
        let ret: ::core::cmp::Ordering = x.cmp(&y);
        Some(ret)
    }
}