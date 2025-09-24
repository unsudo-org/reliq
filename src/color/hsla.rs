use super::*;

pub type Hsla<const A: u8 = 1, B = usize, C = q::DefaultEngine, D = DefaultEngine> = Color<A, B, HslaMode<A, B, C>, C, D>;

impl<const A: u8, B, C, D> Hsla<A, B, C, D> 
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>  {
    #[inline]
    pub fn h(&self) -> &u16 {
        &self.mode.h
    }

    #[inline]
    pub fn s(&self) -> &q::Unit<A, B, C> {
        &self.mode.s
    }

    #[inline]
    pub fn l(&self) -> &q::Unit<A, B, C> {
        &self.mode.l
    }

    #[inline]
    pub fn a(&self) -> &q::Unit<A, B, C> {
        &self.mode.a
    }
}

impl<const A: u8, B, C, D> From<Hex<A, B, C, D>> for Hsla<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    fn from(value: Hex<A, B, C, D>) -> Self {
        use ops::ToPrim;
        let code: u32 = *value;
        let r: u8 = ((code >> 16) & 0xFF) as u8;
        let g: u8 = ((code >> 8) & 0xFF) as u8;
        let b: u8 = (code & 0xFF) as u8;
        let max: u8 = r.max(g).max(b);
        let min: u8 = r.min(g).min(b);
        let delta: u8 = max - min;
        let l_255: u8 = max.saturating_add(min) >> 1;
        let s_255 = if delta == 0 {
            0
        } else {
            let temp = if l_255 < 128 {
                delta * 255 / (max + min).max(1)
            } else {
                delta * 255 / (510 - max - min).max(1)
            };
            temp.min(255)
        };
        let h_deg: u16 = if delta == 0 {
            0
        } else {
            let hue = if max == r {
                60 * (g as i16 - b as i16) / delta as i16
            } else if max == g {
                60 * (b as i16 - r as i16) / delta as i16 + 120
            } else {
                60 * (r as i16 - g as i16) / delta as i16 + 240
            };
            let h = hue.rem_euclid(360);
            h as u16
        };
        let h = h_deg;
        let s = q::r#as::<1, A, B, _, C>(s_255 * 10);
        let l = q::r#as::<1, A, B, _, C>(l_255 * 10);
        let a = q::as_1::<A, B, _, C>();
        Self {
            
        }
    }
}

pub struct HslaMode<const A: u8 = 1, B = usize, C = q::DefaultEngine>
where
    B: ops::Int,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    h: u16,
    s: q::Unit<A, B, C>,
    l: q::Unit<A, B, C>,
    a: q::Unit<A, B, C>
}

impl<const A: u8, B, C> Mode for HslaMode<A, B, C>
where
    B: ops::Int,
    C: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {}