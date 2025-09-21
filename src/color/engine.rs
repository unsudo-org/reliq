use super::*;

pub struct DefaultEngine;

impl Engine for DefaultEngine {}

pub trait Engine {
    fn hex_to_hsl<const A: u8, B, C, D>(hex: Hex<A, B, C, D>) -> Hsl<A, B, C, D>
    where
        B: ops::Int,
        C: q::Engine,
        D: Engine,
        (): q::SupportedPrecision<A>,
        (): q::SupportedInt<B>,
        (): q::Supported<A, B> {
            
    }

    fn hex_to_hsla<const A: u8, B, C, D>(hex: Hex<A, B, C, D>) -> Hsla<A, B, C, D>
    where
        B: ops::Int,
        C: q::Engine,
        D: Engine,
        (): q::SupportedPrecision<A>,
        (): q::SupportedInt<B>,
        (): q::Supported<A, B>,
        (): q::Supported<1, B> {
        let hsl: Hsl<A, B, C, D> = Self::hex_to_hsl(hex);
        let h: u16 = *hsl.h();
        let s: q::Unit<A, B, C> = *hsl.s();
        let l: q::Unit<A, B, C> = *hsl.l();
        let a: q::Unit<A, B, C> = q::as_1();
        Color {
            mode: HslaMode {
                h,
                s,
                l,
                a
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        }
    }

    fn hex_to_rgb<const A: u8, B, C, D>(hex: Hex<A, B, C, D>) -> Rgb<A, B, C, D>
    where
        B: ops::Int,
        C: q::Engine,
        D: Engine,
        (): q::SupportedPrecision<A>,
        (): q::SupportedInt<B>,
        (): q::Supported<A, B> {
        let code: u32 = *hex;
        let r: u8 = ((code >> 16) & 0xFF) as u8;
        let g: u8 = ((code >> 8) & 0xFF) as u8;
        let b: u8 = (code & 0xFF) as u8;
        Color {
            mode: RgbMode {
                r,
                g,
                b
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        }
    }

    fn hex_to_rgba<const A: u8, B, C, D>(hex: Hex<A, B, C, D>) -> Rgba<A, B, C, D>
    where
        B: ops::Int,
        C: q::Engine,
        D: Engine,
        (): q::SupportedPrecision<A>,
        (): q::SupportedInt<B>,
        (): q::Supported<A, B>,
        (): q::Supported<1, B> {
        let code: u32 = *hex;
        let r: u8 = ((code >> 16) & 0xFF) as u8;
        let g: u8 = ((code >> 8) & 0xFF) as u8;
        let b: u8 = (code & 0xFF) as u8;
        let a: q::Unit<A, B, C> = q::as_1();
        Color {
            mode: RgbaMode {
                r,
                g,
                b,
                a
            },
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData,
            m_2: ::core::marker::PhantomData
        }
    }

    fn hsl_to_hex<const A: u8, B, C, D>(hsl: Hsl<A, B, C, D>) -> Hex<A, B, C, D>
    where
        B: ops::Int,
        C: q::Engine,
        D: Engine,
        (): q::SupportedPrecision<A>,
        (): q::SupportedInt<B>,
        (): q::Supported<A, B> {
        
    }
}

