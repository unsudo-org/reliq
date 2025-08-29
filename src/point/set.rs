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

    pub fn preview_k_step(&self, k: usize) -> Result<Point<A, B, C, D>> {
        let len: usize = self.children.len();
        let mut set: array::Array<_, _> = array::Array::<B, Q<A, C, D>>::default();
        for dimension in 0..B {
            let mut vel_sum: Q<A, C, D> = C::AS_0.into();
            for i in (len - B)..len {
                let prev = *self.children[i - 1].dimension(dimension);
                let curr = *self.children[i].dimension(dimension);
                vel_sum = (vel_sum + (curr - prev)?)?;
            }
            let avg_mag = (vel_sum / B.into())?;
            let last = *self.children[len - 1].dimension(dimension);
            set.push((last + avg_mag)?).ok();
        }
        let ret = Point {
            children: set
        };
        Ok(ret)
    }

    
    /// Predict the next N-D point using linear extrapolation.
    pub fn estimate_next(&self) -> Result<Point<A, B, C, D>> {
        let len = self.children.len();
        assert!(len >= 2, "estimate_next requires at least 2 points");
        let p_prev = &self.children[len - 2];
        let p_last = &self.children[len - 1];

        let mut coords = array::Array::<B, Q<A, C, D>>::default();

        for i in 0..B {
            let x_prev = *p_prev
                .dimension(i)
                .expect("Point dimension out of bounds");
            let x_last = *p_last
                .dimension(i)
                .expect("Point dimension out of bounds");

            let vel = (x_last - x_prev)?;
            let x_next = (x_last + vel)?;
            coords.push(x_next).ok();
        }

        Ok(Point { children: coords })
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