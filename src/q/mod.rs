use super::*;

::modwire::expose!(
    pub engine
        fold
        pi
    pub precision
        scale
        wide_mul
);

pub type Q1<T> = Q<1, T>;
pub type Q2<T> = Q<2, T>;
pub type Q3<T> = Q<3, T>;
pub type Q4<T> = Q<4, T>;
pub type Q5<T> = Q<5, T>;
pub type Q6<T> = Q<6, T>;
pub type Q7<T> = Q<7, T>;
pub type Q8<T> = Q<8, T>;
pub type Q9<T> = Q<9, T>;
pub type Q10<T> = Q<10, T>;
pub type Q11<T> = Q<11, T>;
pub type Q12<T> = Q<12, T>;
pub type Q13<T> = Q<13, T>;
pub type Q14<T> = Q<14, T>;
pub type Q15<T> = Q<15, T>;
pub type Q16<T> = Q<16, T>;
pub type Q17<T> = Q<17, T>;
pub type Q18<T> = Q<18, T>;
pub type Q19<T> = Q<19, T>;
pub type Q20<T> = Q<20, T>;
pub type Q21<T> = Q<21, T>;
pub type Q22<T> = Q<22, T>;
pub type Q23<T> = Q<23, T>;
pub type Q24<T> = Q<24, T>;
pub type Q25<T> = Q<25, T>;
pub type Q26<T> = Q<26, T>;
pub type Q27<T> = Q<27, T>;
pub type Q28<T> = Q<28, T>;
pub type Q29<T> = Q<29, T>;
pub type Q30<T> = Q<30, T>;
pub type Q31<T> = Q<31, T>;
pub type Q32<T> = Q<32, T>;
pub type Q33<T> = Q<33, T>;
pub type Q34<T> = Q<34, T>;
pub type Q35<T> = Q<35, T>;
pub type Q36<T> = Q<36, T>;
pub type Q37<T> = Q<37, T>;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B = usize, C = DefaultEngine>
where
    B: num::Int,
    C: Engine {
    v: B,
    engine: ::core::marker::PhantomData<C>
}