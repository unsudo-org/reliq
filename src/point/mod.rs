use super::*;

::modwire::expose!(
    pub error
    pub set
    pub ty
);

type Q<const A: u8, B, C> = q::Q<A, B, q::DefaultMode, C>;

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Point<const A: u8, const B: usize, C, D = q::DefaultEngine>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    children: array::Array<B, Q<A, C, D>>
}

impl<const A: u8, const B: usize, C, D> Point<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub const fn dimensions(&self) -> usize {
        B
    }

    pub const fn is_empty(&self) -> bool {
        self.children.len() == 0
    }

    pub fn dimension(&self, k: usize) -> Option<&Q<A, C, D>> {
        self.children.get(k).ok()
    }


}

impl<const A: u8, const B: usize, C, D> Point<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    C: ops::Signed,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub fn distance_between(self, rhs: Self) -> Result<Q<A, C, D>> {
        let mut sum: Q<A, C, D> = C::AS_0.into();
        let rhs_iter: array::Iter<_, _> = rhs.children.into_iter();
        for (x_0, x_1) in self.children.into_iter().zip(rhs_iter) {
            let dn: Q<A, C, D> = (x_0 - x_1)?;
            let dn: Q<A, C, D> = (dn * dn)?;
            sum = (sum + dn)?;
        }
        let ret: Q<A, C, D> = sum.sqrt()?;
        Ok(ret)
    }

    pub fn nearest(self, set: Set<A, B, C, D>) -> Result<Option<(Q<A, C, D>, Self)>> {
        let mut best: Option<(Q<A, C, D>, Self)> = None;

        for candidate in set.children.into_iter() {
            let distance = self.distance_between(candidate)?;
            match &mut best {
                None => best = Some((distance, candidate)),
                Some((best_dist, best_point)) => {
                    if distance < *best_dist {
                        *best_dist = distance;
                        *best_point = candidate;
                    }
                }
            }
        }

        Ok(best)
    }
}

impl<const A: u8, const B: usize, C, D> From<array::Array<B, Q<A, C, D>>> for Point<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(children: array::Array<B, Q<A, C, D>>) -> Self {
        Self {
            children
        }
    }
}
