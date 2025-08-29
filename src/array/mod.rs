::modwire::expose!(
    pub iter
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
pub enum Error {
    Overflow,
    KeyOutOfBounds,
    Empty
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
    pub const fn get(&self, key: usize) -> Result<&B> {
        if key >= self.len {
            return Err(Error::KeyOutOfBounds)
        }
        unsafe {
            let ret: &B = &*self.buf[key].as_ptr();
            Ok(ret)
        }
    }

    #[inline]
    pub const fn get_mut(&mut self, key: usize) -> Result<&mut B> {
        if key >= self.len {
            return Err(Error::KeyOutOfBounds)
        }
        unsafe {
            let ret: &mut B = &mut *self.buf[key].as_mut_ptr();
            Ok(ret)
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
    pub const fn push(&mut self, item: B) -> Result<()> {
        if self.len >= A {
            return Err(Error::Overflow)
        }
        self.buf[self.len].write(item);
        self.len += 1;
        Ok(())
    }

    #[inline]
    pub const fn pop(&mut self) -> Result<B> {
        if self.len == 0 {
            return Err(Error::Empty)
        }
        self.len -= 1;
        unsafe {
            let ret: B = self.buf[self.len].assume_init_read();
            Ok(ret)
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
    pub const fn swap_insert(&mut self, key: usize, data: B) -> Result<()> {
        if self.len >= A {
            return Err(Error::Overflow)
        }
        if key > self.len {
            return Err(Error::KeyOutOfBounds)
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
        Ok(())
    }

    #[inline]
    pub const fn swap_remove(&mut self, key: usize) -> Result<B> {
        if key >= self.len {
            return Err(Error::KeyOutOfBounds)
        }
        let ret: B = unsafe {
            self.buf[key].assume_init_read()
        };
        if key != self.len - 1 {
            self.buf[key] = self.buf[self.len - 1];
        }
        self.len -= 1;
        Ok(ret)
    }    

    #[inline]
    pub fn insert(&mut self, key: usize, item: B) -> Result<()> {
        if self.len >= A {
            return Err(Error::Overflow)
        }
        if self.len <= key {
            return Err(Error::KeyOutOfBounds)
        }
        for i in (key..self.len).rev() {
            self.buf[i + 1] = self.buf[i];
        }
        self.buf[key].write(item);
        self.len += 1;
        Ok(())
    }

    #[inline]
    pub fn remove(&mut self, key: usize) -> Result<B> {
        if self.len == 0 {
            return Err(Error::Empty)
        }
        if self.len <= key {
            return Err(Error::KeyOutOfBounds)
        }
        let item: B = unsafe {
            self.buf[key].assume_init_read()
        };
        for i in key..self.len - 1 {
            self.buf[i] = self.buf[i + 1];
        }
        self.len -= 1;
        Ok(item)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_pop() {
        let mut arr: Array<4, u8> = Array::default();
        let len: usize  = arr.len();
        assert_eq!(len, 0);
        let item_0: u8 = 2;
        let item_1: u8 = 4;
        arr.push(item_0).unwrap();
        arr.push(item_1).unwrap();
        let len: usize = arr.len();
        assert_eq!(len, 2);
        let arr_item_0: &u8 = arr.get(0).unwrap();
        let arr_item_1: &u8 = arr.get(1).unwrap();
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
    fn insert_remove_ordered() {
        let mut arr: Array<4, u8> = Array::default();
        arr.push(1).unwrap();
        arr.push(3).unwrap();
        arr.insert(1, 2).unwrap();
        assert_eq!(arr.as_slice(), &[1, 2, 3]);
        let val = arr.remove(1).unwrap();
        assert_eq!(val, 2);
        assert_eq!(arr.as_slice(), &[1, 3]);
    }

    #[test]
    fn swap_insert_remove_unordered() {
        let mut arr: Array<4, u8> = Array::default();
        arr.push(10).unwrap();
        arr.push(20).unwrap();
        arr.push(30).unwrap();
        arr.swap_insert(1, 15).unwrap();
        assert_eq!(arr.len(), 4);
        assert!(arr.as_slice().contains(&15));

        let val = arr.swap_remove(1).unwrap();
        assert!(val == 15 || val == 20);
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn iter() {
        let mut arr: Array<3, u8> = Array::default();
        arr.push(1).unwrap();
        arr.push(2).unwrap();
        arr.push(3).unwrap();
        let collected: Array<3, u8> = arr.into_iter().collect();
        assert_eq!(collected, array!(1, 2, 3));
    }
}