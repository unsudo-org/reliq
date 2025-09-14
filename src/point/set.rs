use super::*;

#[repr(transparent)]
#[derive(Clone)]
pub struct Set<const A: u8, const B: usize, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub(crate) points: array::Array<B, Point<A, B, C, D>>
}

impl<const A: u8, const B: usize, C, D> Set<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    pub fn next_weighted(&self) -> Result<Point<A, B, C, D>> {
        let len: usize = self.points.len();
        if len < 3 {
            return self.next_linear()
        }
        let mut dimensions: array::Array<B, Q<A, C, D>> = array::Array::default();
        let l: &Point<A, B, C, D> = self.points.get(len - 1).unwrap();
        let m: &Point<A, B, C, D> = self.points.get(len - 2).unwrap();
        let p: &Point<A, B, C, D> = self.points.get(len - 3).unwrap();
        for i in 0..B {
            let p: Q<A, C, D> = *p.dimension(i).unwrap();
            let m: Q<A, C, D> = *m.dimension(i).unwrap();
            let l: Q<A, C, D> = *l.dimension(i).unwrap();
            let a: Q<A, C, D> = (m - p)?;
            let b: Q<A, C, D> = (l - m)?;
            let c: Q<A, C, D> = C::AS_2.into();
            let d: Q<A, C, D> = C::AS_3.into();
            let next: Q<A, C, D> = ((l + ((a + (b * c)?)?))? / d)?;
            dimensions.push(next).ok();
        }
        let ret: Point<A, B, C, D> = dimensions.into();
        Ok(ret)
    }

    #[inline]
    pub fn next_k_step(&self) -> Result<Point<A, B, C, D>> {
        let len: usize = self.points.len();
        let mut arr: array::Array<B, Q<A, C, D>> = array::Array::default();
        for dimension in 0..B {
            let mut magnitude_sum: Q<A, C, D> = C::AS_0.into();
            for i in (len - B)..len {
                let prev: Point<A, B, C, D> = *self.points.get(i - 1)?;
                let prev: Q<A, C, D> = *prev.dimension(dimension).expect("Point dimension is not out of bounds.");
                let curr: Point<A, B, C, D> = *self.points.get(i)?;
                let curr: Q<A, C, D> = *curr.dimension(dimension).expect("Point dimension is not out of bounds.");
                magnitude_sum = (magnitude_sum + (curr - prev)?)?;
            }
            let len: usize = B;
            let len: C = len
                .try_into()
                .ok()
                .expect("???");
            let len: Q<A, C, D> = len.into();
            let magnitude_average: Q<A, C, D> = (magnitude_sum / len)?;
            let k: C = len.as_int();
            let k: C = k - C::AS_1;
            let k: usize = k
                .try_into()
                .ok()
                .expect("???");
            let last: Point<A, B, C, D> = *self.points.get(k).unwrap();
            let last: Q<A, C, D> = *last.dimension(dimension).unwrap();
            arr.push((last + magnitude_average)?).ok();
        }
        let ret: Point<A, B, C, D> = Point {
            dimensions: arr
        };
        Ok(ret)
    }

    #[inline]
    pub fn next_linear(&self) -> Result<Point<A, B, C, D>> {
        let len: usize = self.points.len();
        if len < 2 {
            return Err(Error::InsufficientSetSize)
        }
        let prev: &Point<A, B, C, D> = self.points.get(len - 2)?;
        let last: &Point<A, B, C, D> = self.points.get(len - 1)?;
        let mut arr: array::Array<B, Q<A, C, D>> = array::Array::default();
        for i in 0..B {
            let prev_x: Q<A, C, D> = *prev.dimension(i).expect("Point dimension is not out of bounds.");
            let last_x: Q<A, C, D> = *last.dimension(i).expect("Point dimension is not out of bounds.");
            let magnitude: Q<A, C, D> = (last_x - prev_x)?;
            let next_x: Q<A, C, D> = (last_x + magnitude)?;
            arr.push(next_x).ok();
        }
        let ret: Point<A, B, C, D> = arr.into();
        Ok(ret)
    }
}

impl<const A: u8, const B: usize, C, D, E> From<array::Array<B, E>> for Set<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    E: Into<Point<A, B, C, D>>,
    E: Copy,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(value: array::Array<B, E>) -> Self {
        let mut points: array::Array<B, Point<A, B, C, D>> = array::Array::default();
        for item in value {
            let item: Point<A, B, C, D> = item.into();
            points.push(item).ok();
        }
        Self {
            points
        }
    }
}

impl<const A: u8, const B: usize, C, D, E> From<[E; B]> for Set<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    E: Into<Point<A, B, C, D>>,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(value: [E; B]) -> Self {
        let value: [Point<A, B, C, D>; B] = value.map(|item| {
            let item: Point<A, B, C, D> = item.into();
            item
        });
        let points: array::Array<B, Point<A, B, C, D>> = value.into();
        Self {
            points
        }
    }
}

impl<const A: u8, const B: usize, C, D> ::core::ops::Deref for Set<A, B, C, D> 
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    type Target = array::Array<B, Point<A, B, C, D>>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

impl<const A: u8, const B: usize, C, D> ::core::ops::DerefMut for Set<A, B, C, D> 
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points
    }
}

#[test]
fn test_next_linear() {
    let mut set: Set<2, 2, u128, q::DefaultEngine> = [[
        0_00, 
        0_00
    ], [
        0_00, 
        0_00
    ]].into();
    set.push(
        [
            1_00,
            2_00
        ].into()
    ).ok();
    let predicted_point: Point<2, 2, u128> = set.next_linear().unwrap();
    let expected_predicted_point: Point<2, 2, u128> = [
        0_00,
        0_00
    ].into();
    assert_eq!(predicted_point, expected_predicted_point);
}