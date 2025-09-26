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
pub struct Point<const A: u8, const B: usize, C>
where
    C: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    dimensions: array::Array<B, q::Unit<A, C>>
}

impl<const A: u8, const B: usize, C> Point<A, B, C>
where
    C: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub const fn len(&self) -> usize {
        B
    }

    pub fn dimension(&self, key: usize) -> Option<&q::Unit<A, C>> {
        self.dimensions.get(key)
    }

    pub fn x(&self) -> Option<&q::Unit<A, C>> {
        self.dimension(0)
    }

    pub fn y(&self) -> Option<&q::Unit<A, C>> {
        self.dimension(1)
    }

    pub fn z(&self) -> Option<&q::Unit<A, C>> {
        self.dimension(2)
    }
}

impl<const A: u8, const B: usize, C> Point<A, B, C>
where
    C: ops::Int,
    C: ops::Unsigned,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub fn distance_between(self, rhs: Self) -> Result<q::Unit<A, C>> {
        let mut sum: q::Unit<A, C> = q::r#as::<2, _, u16, _, _, _>(0_u16).unwrap();
        let rhs_iter: array::Iter<B, q::Unit<A, C>> = rhs.dimensions.into_iter();
        for (x_0, x_1) in self.dimensions.into_iter().zip(rhs_iter) {
            let dn: q::Unit<A, C> = (x_0 - x_1)?;
            let dn_sq: q::Unit<A, C> = (dn * dn)?;
            sum = (sum + dn_sq)?;
        }
        let ret: q::Unit<A, C> = sum.sqrt()?;
        Ok(ret)
    }

    pub fn nearest<E>(self, points: E) -> Result<Option<(q::Unit<A, C>, Point<A, B, C>)>> 
    where
        E: Into<array::Array<B, Point<A, B, C>>> {
        let points: array::Array<B, Point<A, B, C>> = points.into();
        let mut best: Option<(q::Unit<A, C>, Point<A, B, C>)> = None;
        for point in points.into_iter() {
            let distance: q::Unit<A, B> = self.distance_between(point)?;
            match &mut best {
                None => best = Some((distance, point)),
                Some((best_distance, best_point)) => {
                    if distance < *best_distance {
                        *best_distance = distance;
                        *best_point = point;
                    }
                }
            }
        }
        Ok(best)
    }
}

impl<const A: u8, const B: usize, C> Default for Point<A, B, C>
where
    C: ops::Int,
    C: ops::Prim,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn default() -> Self {
        let mut dimensions: array::Array<B, q::Q<A, C>> = array::Array::default();
        for _ in 0..B {
            let zero: q::Q<A, C> = C::AS_0.into();
            dimensions.push(zero).unwrap();
        }
        Self {
            dimensions: array::Array::default()
        }
    }
}

impl<const A: u8, const B: usize, C, D> From<array::Array<B, D>> for Point<A, B, C>
where
    C: ops::Int,
    C: ops::Prim,
    D: Into<q::Q<A, C>>,
    D: Copy,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(value: array::Array<B, D>) -> Self {
        let mut dimensions: array::Array<B, q::Q<A, C>> = array::Array::default();
        for item in value {
            let item: q::Q<A, C> = item.into();
            dimensions.push(item).ok();
        }
        Self {
            dimensions
        }
    }
}

impl<const A: u8, const B: usize, C, E> From<[E; B]> for Point<A, B, C>
where
    C: ops::Int,
    C: ops::Prim,
    E: Into<q::Q<A, C>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(value: [E; B]) -> Self {
        let value: [q::Q<A, C>; B] = value.map(|item| {
            item.into()
        });
        let dimensions: array::Array<B, q::Q<A, C>> = value.into();
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