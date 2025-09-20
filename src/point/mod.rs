use super::*;

::modwire::expose!(
    pub error
    pub set
    pub ty
);

pub type Dimensions = usize;

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub struct Point<
    const A: u8, 
    const B: usize, 
    C, 
    D = q::DefaultEngine
>
where
    C: ops::Int,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    dimensions: array::Array<B, q::Unit<A, C, D>>
}

impl<
    const A: u8, 
    const B: usize, 
    C, 
    D
> Point<A, B, C, D>
where
    C: ops::Int,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub const fn len(&self) -> usize {
        B
    }

    pub fn dimension(&self, key: usize) -> Option<&q::Unit<A, C, D>> {
        self.dimensions.get(key)
    }

    pub fn x(&self) -> Option<&q::Unit<A, B, D>> {
        self.dimension(0)
    }

    pub fn y(&self) -> Option<&q::Unit<A, B, C>> {
        self.dimension(1)
    }

    pub fn z(&self) -> Option<&q::Unit<A, B, C>> {
        self.dimension(2)
    }
}

impl<const A: u8, const B: usize, C, D> Point<A, B, C, D>
where
    C: ops::Int,
    C: ops::Signed,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub fn distance_between(self, rhs: Self) -> Result<q::Unit<A, C, D>> {
        let mut sum: q::Unit<A, C, D> = C::AS_0.into();
        let rhs_iter: array::Iter<_, _> = rhs.dimensions.into_iter();
        for (x_0, x_1) in self.dimensions.into_iter().zip(rhs_iter) {
            let dn: Q<A, C, D> = (x_0 - x_1)?;
            let dn: Q<A, C, D> = (dn * dn)?;
            sum = (sum + dn)?;
        }
        let ret: Q<A, C, D> = sum.sqrt()?;
        Ok(ret)
    }

    pub fn nearest<E>(self, set: E) -> Result<Option<(Q<A, C, D>, Self)>> 
    where
        E: Into<Set<A, B, C, D>> {
        let set: Set<A, B, C, D> = set.into();
        let mut best: Option<(Q<A, C, D>, Self)> = None;
        for candidate in set.points.into_iter() {
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

impl<const A: u8, const B: usize, C, D> Default for Point<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn default() -> Self {
        let mut dimensions: array::Array<B, Q<A, C, D>> = array::Array::default();
        for _ in 0..B {
            let zero: Q<A, C, D> = C::AS_0.into();
            dimensions.push(zero).unwrap();
        }
        Self {
            dimensions: array::Array::default()
        }
    }
}

impl<const A: u8, const B: usize, C, D, E> From<array::Array<B, E>> for Point<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    E: Into<Q<A, C, D>>,
    E: Copy,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(value: array::Array<B, E>) -> Self {
        let mut dimensions: array::Array<B, Q<A, C, D>> = array::Array::default();
        for item in value {
            let item: Q<A, C, D> = item.into();
            dimensions.push(item).ok();
        }
        Self {
            dimensions
        }
    }
}

impl<const A: u8, const B: usize, C, D, E> From<[E; B]> for Point<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    E: Into<Q<A, C, D>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(value: [E; B]) -> Self {
        let value: [Q<A, C, D>; B] = value.map(|item| {
            item.into()
        });
        let dimensions: array::Array<B, Q<A, C, D>> = value.into();
        Self {
            dimensions
        }
    }
}

impl<
    const A: q::Precision,
    const B: Dimensions,
    C,
    D
> IntoIterator for Point<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    type Item = q::Unit<A, C, D>;
    type IntoIter = array::Iter<B, q::Unit<A, C, D>>;

    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

fn t(coordinate: Point<2, 32, usize>) {
    for dim in coordinate {
        
    }
}