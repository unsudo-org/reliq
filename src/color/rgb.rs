use super::*;

pub type Rgb<
    const A: q::Precision, 
    B = usize, 
    C = q::DefaultEngine, 
    D = DefaultEngine
> = Color<A, B, RgbMode, C, D>;

pub struct RgbMode {
    pub(super) r: u8,
    pub(super) g: u8,
    pub(super) b: u8
}

impl Mode for RgbMode {}

impl<
    const A: q::Precision,
    B,
    C,
    D
> Rgb<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {

}

impl<
    const A: q::Precision,
    B,
    C,
    D
> From<Hsl<A, B, C, D>> for Rgb<A, B, C, D>
where
    B: ops::Int,
    C: q::Engine,
    D: Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    #[inline]
    fn from(value: Hsl<A, B, C, D>) -> Self {
        D::hsl_to_rgb::<A, B, C, D>(value)
    }
}