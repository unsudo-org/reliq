use super::*;
use ::core::result;
use ::core::cmp;
use ::core::mem;

::modwire::expose!(
    pub iter
);

pub type Result<T> = result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
pub enum Error {
    Overflow,
    KeyOutOfBounds,
    Empty
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Array<const A: usize, B>
where
    B: Copy {
    pub(super) buf: [mem::MaybeUninit<B>; A],
    pub(super) len: usize
}

impl<const A: usize, B> Array<A, B> 
where
    B: Copy {
    #[inline]
    pub fn new(data: [B; A]) -> Self {
        let mut buf: [mem::MaybeUninit<B>; A] = unsafe {
            mem::MaybeUninit::uninit().assume_init()
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
    pub const fn push(&mut self, data: B) -> Result<()> {
        if self.len >= A {
            return Err(Error::Overflow)
        }
        self.buf[self.len].write(data);
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
}

