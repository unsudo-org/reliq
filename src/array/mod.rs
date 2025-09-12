use super::*;

::modwire::expose!(
    pub handle
    pub iter
    pub tracker
);

#[macro_export]
macro_rules! array {
    ($($data:expr),* $(,)?) => {{
        let mut arr = Array::<{count!($($data),*)}, _>::default();
        $(
            arr.push($data).expect("Exceeded fixed capacity.");
        )*
        arr
    }};
}

#[allow(unused_macros)]
macro_rules! count {
    () => {
        0
    };
    ($head:expr $(,$tail:expr)*) => {
        1 + count!($($tail),*)
    };
}

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIs)]
#[derive(::thiserror::Error)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub enum Error {
    #[error("Overflow")]
    Overflow,
    #[error("Key out of bounds")]
    KeyOutOfBounds
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Array<const A: usize, B>
where
    B: Clone,
    B: Copy {
    pub(super) buf: [::core::mem::MaybeUninit<B>; A],
    pub(super) len: usize
}

impl<const A: usize, B> Array<A, B> 
where
    B: Copy {
    #[inline]
    pub fn new(data: [B; A]) -> Self {
        let mut buf: [::core::mem::MaybeUninit<B>; A] = unsafe {
            ::core::mem::MaybeUninit::uninit().assume_init()
        };
        for (k, data) in data.into_iter().enumerate() {
            buf[k].write(data);
        }
        Self {
            buf,
            len: A
        }
    }

    #[inline]
    pub fn get<C>(&self, key: C) -> Option<&B> 
    where
        C: Into<usize> {
        let key: usize = key.into();
        if key >= self.len {
            return None
        }
        unsafe {
            let ret: &B = &*self.buf[key].as_ptr();
            Some(ret)
        }
    }

    #[inline]
    pub fn get_mut<C>(&mut self, key: C) -> Option<&mut B> 
    where
        C: Into<usize> {
        let key: usize = key.into();
        if key >= self.len {
            return None
        }
        unsafe {
            let ret: &mut B = &mut *self.buf[key].as_mut_ptr();
            Some(ret)
        }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn cap(&self) -> usize {
        A
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn push<C>(&mut self, item: C) -> Result<()> 
    where
        C: Into<B> {
        let item: B = item.into();
        if self.len >= A {
            return Err(Error::Overflow)
        }
        self.buf[self.len].write(item);
        self.len += 1;
        Ok(())
    }

    #[inline]
    pub const fn pop(&mut self) -> Option<B> {
        if self.len == 0 {
            return None
        }
        self.len -= 1;
        unsafe {
            let ret: B = self.buf[self.len].assume_init_read();
            Some(ret)
        }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[B] {
        let data: *const B = self.buf.as_ptr() as *const B;
        unsafe {
            ::core::slice::from_raw_parts(data, self.len)
        }
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [B] {
        let data: *mut B = self.buf.as_mut_ptr() as *mut B;
        unsafe {
            ::core::slice::from_raw_parts_mut(data, self.len)
        }
    }

    #[inline]
    pub fn swap_insert<C, D>(&mut self, key: C, item: D) -> Option<Result<()>> 
    where
        C: Into<usize>,
        D: Into<B> {
        let key: usize = key.into();
        let data: B = item.into();
        if key > self.len {
            return None
        }
        if self.len >= A {
            return Some(Err(Error::Overflow))
        }
        self.buf[self.len].write(data);
        self.len += 1;
        if key != self.len - 1 {
            unsafe {
                let temporary: B = self.buf[key].assume_init_read();
                self.buf[key] = self.buf[self.len - 1];
                self.buf[self.len - 1].write(temporary);
            }
        }
        Some(Ok(()))
    }

    #[inline]
    pub fn swap_remove(&mut self, key: usize) -> Option<B> {
        if key >= self.len {
            return None
        }
        let ret: B = unsafe {
            self.buf[key].assume_init_read()
        };
        if key != self.len - 1 {
            self.buf[key] = self.buf[self.len - 1];
        }
        self.len -= 1;
        Some(ret)
    }    

    #[inline]
    pub fn insert<C, D>(&mut self, key: C, item: D) -> Option<Result<()>> 
    where
        C: Into<usize>,
        D: Into<B> {
        let key: usize = key.into();
        let item: B = item.into();
        if self.len <= key {
            return None
        }
        if self.len >= A {
            return Some(Err(Error::Overflow))
        }
        for i in (key..self.len).rev() {
            self.buf[i + 1] = self.buf[i];
        }
        self.buf[key].write(item);
        self.len += 1;
        Some(Ok(()))
    }

    #[inline]
    pub fn remove<C>(&mut self, key: C) -> Option<B> 
    where
        C: Into<usize> {
        let key: usize = key.try_into().ok()?;
        if self.len == 0 || self.len <= key {
            return None
        }
        let item: B = unsafe {
            self.buf[key].assume_init_read()
        };
        for i in key..self.len - 1 {
            self.buf[i] = self.buf[i + 1];
        }
        self.len -= 1;
        Some(item)
    }
}

impl<const A: usize, B> Default for Array<A, B>
where
    B: Copy {
    #[inline]
    fn default() -> Self {
        Self {
            buf: unsafe {
                ::core::mem::MaybeUninit::uninit().assume_init()
            },
            len: 0
        }
    }
}

impl<const A: usize, B, C> From<[C; A]> for Array<A, B>
where
    B: Copy,
    C: Into<B> {
    fn from(value: [C; A]) -> Self {
        let value: [B; A] = value.map(|item| {
            item.into()
        });
        Self::new(value)
    }
}

impl<const A: u8, const B: usize, C, D> From<point::Set<A, B, C, D>> for Array<B, point::Point<A, B, C, D>>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(value: point::Set<A, B, C, D>) -> Self {
        value.points
    }
}

impl<const A: u8, const B: usize, C, D> From<point::Point<A, B, C, D>> for Array<B, q::Q<A, C, q::DefaultMode, D>>
where
    C: ops::Int,
    C: ops::Prim,
    D: q::Engine,
    (): q::SupportedPrecision<A>,
    (): q::SupportedInt<C>,
    (): q::Supported<A, C> {
    fn from(value: point::Point<A, B, C, D>) -> Self {
        value.dimensions
    }
}

#[cfg(feature = "std")]
impl<const A: usize, B, C> TryFrom<Vec<C>> for Array<A, B>
where
    B: Copy,
    C: Into<B> {
    type Error = Error;

    fn try_from(value: Vec<C>) -> ::core::result::Result<Self, Self::Error> {
        let mut ret: Self = Self::default();
        for item in value {
            let item: B = item.into();
            ret.push(item)?;
        }
        Ok(ret)
    }
}

impl<const A: usize, B> Eq for Array<A, B>
where
    B: Copy,
    B: PartialEq {}

impl<const A: usize, B> PartialEq for Array<A, B>
where
    B: Copy,
    B: PartialEq {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.as_slice() == other.as_slice()
    }
}

impl<const A: usize, B> FromIterator<B> for Array<A, B>
where
    B: Copy {
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        let mut arr: Self = Self::default();
        for item in iter {
            if arr.push(item).is_err() {
                break
            }
        }
        arr
    }
}

impl<const A: usize, B> IntoIterator for Array<A, B>
where
    B: Copy {
    type Item = B;
    type IntoIter = Iter<A, B>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            buf: self.buf,
            len: self.len,
            key: 0
        }
    }
}

impl<const A: usize, B> ::core::hash::Hash for Array<A, B> 
where
    B: Copy,
    B: ::core::hash::Hash {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        for byte in self.as_slice() {
            byte.hash(state);
        }
    }
}

impl<const A: usize, B> ::serde::Serialize for Array<A, B>
where
    B: Copy,
    B: ::serde::Serialize {
    fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        self.as_slice().serialize(serializer)
    }
}

impl<'de, const A: usize, B> ::serde::Deserialize<'de> for Array<A, B>
where
    B: Copy,
    B: ::serde::Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        use serde::de::{self, Deserializer, SeqAccess, Visitor};
        use core::fmt;

        struct ArrayVisitor<const N: usize, B> {
            marker: core::marker::PhantomData<B>,
        }

        impl<'de, const N: usize, B> Visitor<'de> for ArrayVisitor<N, B>
        where
            B: Copy + serde::Deserialize<'de> {
            type Value = Array<N, B>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array of at most {} elements", N)
            }

            fn visit_seq<A>(self, mut seq: A) -> ::core::result::Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut arr = Array::<N, B>::default();
                while let Some(value) = seq.next_element::<B>()? {
                    arr.push(value)
                        .map_err(|_| de::Error::custom("array overflow"))?;
                }
                Ok(arr)
            }
        }

        deserializer.deserialize_seq(ArrayVisitor::<A, B> { marker: core::marker::PhantomData })
    }
}

#[test]
fn test_push_pop() {
    let mut arr: Array<4, u8> = Array::default();
    let len: usize  = arr.len();
    assert_eq!(len, 0);
    let item_0: u8 = 2;
    let item_1: u8 = 4;
    arr.push(item_0).unwrap();
    arr.push(item_1).unwrap();
    let len: usize = arr.len();
    assert_eq!(len, 2);
    let arr_item_0: &u8 = arr.get(0_usize).unwrap();
    let arr_item_1: &u8 = arr.get(1_usize).unwrap();
    assert_eq!(arr_item_0, &item_0);
    assert_eq!(arr_item_1, &item_1);
    let arr_item_1: u8 = arr.pop().unwrap();
    let arr_item_0: u8 = arr.pop().unwrap();
    assert_eq!(arr_item_0, item_0);
    assert_eq!(arr_item_1, item_1);
    let len: usize = arr.len();
    assert_eq!(len, 0);
}

#[test]
fn test_insert_remove_ordered() {
    let mut arr: Array<4, u8> = Array::default();
    arr.push(1).unwrap();
    arr.push(3).unwrap();
    arr.insert(1_usize, 2).unwrap();
    assert_eq!(arr.as_slice(), &[1, 2, 3]);
    let val = arr.remove(48_usize).unwrap();
    assert_eq!(val, 2);
    assert_eq!(arr.as_slice(), &[1, 3]);
}

#[test]
fn test_swap_insert_remove_unordered() {
    let mut arr: Array<4, u8> = Array::default();
    arr.push(10).unwrap();
    arr.push(20).unwrap();
    arr.push(30).unwrap();
    arr.swap_insert(1usize, 15).unwrap();
    assert_eq!(arr.len(), 4);
    assert!(arr.as_slice().contains(&15));

    let val = arr.swap_remove(1).unwrap();
    assert!(val == 15 || val == 20);
    assert_eq!(arr.len(), 3);
}

#[test]
fn test_iter() {
    let mut arr: Array<3, u8> = Array::default();
    arr.push(1).unwrap();
    arr.push(2).unwrap();
    arr.push(3).unwrap();
    let collected: Array<3, u8> = arr.into_iter().collect();
    assert_eq!(collected, array!(1, 2, 3));
}