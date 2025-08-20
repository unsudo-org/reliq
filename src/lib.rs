#![no_std]

pub mod q;
pub mod num;

fn do_something<const T: u8>(count: q::Q<T, u16>) 
where
    (): q::Precision<T>,
    (): q::ScaleCompatible<T, u16>,
    (): q::PICompatible<T, u16> {
    
}