use super::*;

::modwire::expose!(
    pub mode
    pub engine
        fold
        n
        pi
    pub precision
        scale
        trig
        wide_mul
);

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B = usize, C = DefaultMode, D = DefaultEngine>
where
    B: num::Int,
    C: Mode,
    D: Engine,
    (): Precision<A>,
    (): N<B> {
    n: B,
    m_0: ::core::marker::PhantomData<C>,
    m_1: ::core::marker::PhantomData<D>
}

impl<const A: u8, B, C, D> Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    pub fn cast<const E: u8>(self) -> Result<Q<E, B, C, D>> 
    where
        (): Precision<E> {
        let n: B = self.n;
        let n: B = D::cast::<A, E, _>(n)?;
        let n: Q<E, B, C, D> = Q {
            n,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        };
        Ok(n)
    }
}

impl<const A: u8, B, C, D> Eq for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine,
    (): Precision<A>,
    (): N<B> 
    {}

impl<const A: u8, B, C, D> PartialEq for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl<const A: u8, B, C, D> Ord for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        let x: B = self.n;
        let y: B = other.n;
        if x > y {
            ::core::cmp::Ordering::Greater
        } else if x < y {
            ::core::cmp::Ordering::Less
        } else {
            ::core::cmp::Ordering::Equal
        }
    }
}

impl<const A: u8, B, C, D> PartialOrd for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine,
    (): Precision<A>,
    (): N<B> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}