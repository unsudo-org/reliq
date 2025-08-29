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
    pub(super) children: array::Array<B, Point<A, B, C, D>>
}

impl<const A: u8, const B: usize, C, D> Set<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    #[inline]
    pub fn predict_next_via_k_step(&self) -> Result<Point<A, B, C, D>> {
        let len: usize = self.children.len();
        let mut arr: array::Array<B, Q<A, C, D>> = array::Array::default();
        for dimension in 0..B {
            let mut magnitude_sum: Q<A, C, D> = C::AS_0.into();
            for i in (len - B)..len {
                let prev: Point<A, B, C, D> = *self.children.get(i - 1)?;
                let prev: Q<A, C, D> = *prev.dimension(dimension).expect("Point dimension is not out of bounds.");
                let curr: Point<A, B, C, D> = *self.children.get(i)?;
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
            let k: C = len.to_int();
            let k: C = k - C::AS_1;
            let k: usize = k
                .try_into()
                .ok()
                .expect("???");
            let last: Point<A, B, C, D> = *self.children.get(k).unwrap();
            let last: Q<A, C, D> = *last.dimension(dimension).unwrap();
            arr.push((last + magnitude_average)?).ok();
        }
        let ret: Point<A, B, C, D> = Point {
            children: arr
        };
        Ok(ret)
    }

    /// Predict the next N-D point using linear extrapolation.
    #[inline]
    pub fn predict_next_via_linear(&self) -> Result<Point<A, B, C, D>> {
        let len: usize = self.children.len();
        if len < 2 {
            return Err(Error::InsufficientSetSize)
        }
        let prev: &Point<A, B, C, D> = self.children.get(len - 2)?;
        let last: &Point<A, B, C, D> = self.children.get(len - 1)?;
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

impl<const A: u8, const B: usize, C, D> From<array::Array<B, Point<A, B, C, D>>> for Set<A, B, C, D>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(value: array::Array<B, Point<A, B, C, D>>) -> Self {
        Self {
            children: value
        }
    }
}