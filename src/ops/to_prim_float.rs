use super::*;

pub trait ToFloat {
    fn to_float<T>(&self) -> T
    where
        T: Float,
        T: Prim;
}

impl ToFloat for u8 {
    fn to_float<T>(&self) -> T
    where
        T: Float,
        T: Prim {
        match T::VARIANT {
            PrimVariant::F32 => {
                let n: f32 = *self as f32;
                unsafe {
                    ::core::mem::transmute_copy::<f32, T>(&n)
                }
            },
            PrimVariant::F64 => {
                let n: f64 = *self as f64;
                unsafe {
                    ::core::mem::transmute_copy::<f64, T>(&n)
                }
            },
            _ => {
                unsafe {
                    ::core::hint::unreachable_unchecked()
                }
            }
        }
    }
}

impl ToFloat for f32 {
    fn to_float<T>(&self) -> T
    where
        T: Float {
        
    }
}