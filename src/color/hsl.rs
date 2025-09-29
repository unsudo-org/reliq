use super::*;

pub struct HslMode<const A: u8, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    h: u16,
    s: q::Unit<A, B>,
    l: q::Unit<A, B>
}

impl<const A: u8, B> Mode for HslMode<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B>
    {}

pub type Hsl<const A: u8 = 1, B = usize> = Color<A, B, HslMode<A, B>>;

impl<const A: u8, B> Hsl<A, B> 
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    pub fn h(&self) -> u16 {
        self.mode.h
    }

    pub fn s(&self) -> q::Unit<A, B> {
        self.mode.s
    }

    pub fn l(&self) -> q::Unit<A, B> {
        self.mode.l
    }
}

impl<const A: u8, B> TryFrom<Rgb<A, B>> for Hsl<A, B>
where
    B: ops::Int,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<B>,
    (): q::Supported<A, B> {
    type Error = Error;

    fn try_from(value: Rgb<A, B>) -> ::core::result::Result<Self, Self::Error> {
        let rgb: Rgb<A, B> = value;
        let r: u8 = rgb.r();
        let g: u8 = rgb.g();
        let b: u8 = rgb.b();
        let n255: B = 255.try_into()?;
        let n255: q::Unit<A, B> = n255.into();
        let rf: B = r
            .try_into()
            .ok()
            .ok_or(Error::UnsupportedConversion)?;
        let rf: q::Unit<A, B> = rf.into();
        let rf: q::Unit<A, B> = (rf / n255)?;
        let gf: B = g
            .try_into()
            .ok()
            .ok_or(Error::UnsupportedConversion)?;
        let gf: q::Unit<A, B> = gf.into();
        let gf: q::Unit<A, B> = (gf / n255)?;
        let bf: B = b
            .try_into()
            .ok()
            .ok_or(Error::UnsupportedConversion)?;
        let bf: q::Unit<A, B> = bf.into();
        let bf: q::Unit<A, B> = (bf / n255)?;
        let max: q::Unit<A, B> = rf.max(gf).max(bf);
        let min: q::Unit<A, B> = rf.min(gf).min(bf);
    }
}