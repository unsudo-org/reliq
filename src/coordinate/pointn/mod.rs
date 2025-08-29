use super::*;

type Q<const A: u8, B, C> = q::Q<A, B, q::DefaultMode, C>;

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct PointN<const A: u8, const B: usize, C, D = q::DefaultEngine>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    n: array::Array<B, Q<A, C, D>>
}

impl<const A: u8, const B: usize, C, D> PointN<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub fn dimension(&self, k: usize) -> Option<&Q<A, C, D>> {
        self.n.get(k).ok()
    }
}

impl<const A: u8, const B: usize, C, D> Point<A, C, D> for PointN<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    C: ops::Signed,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn distance_between(self, rhs: Self) -> Result<Q<A, C, D>> {
        let mut sum: Q<A, C, D> = C::AS_0.into();
        let rhs_iter: array::Iter<_, _> = rhs.n.into_iter();
        for (x_0, x_1) in self.n.into_iter().zip(rhs_iter) {
            let dn: Q<A, C, D> = (x_0 * x_1)?;
            let dn: Q<A, C, D> = (dn * dn)?;
            sum = (sum + dn)?;
        }
        let ret: Q<A, C, D> = sum.sqrt()?;
        Ok(ret)
    }
}