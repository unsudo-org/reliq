use super::*;

pub type Result<T> = ::core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
pub enum Error {
    Overflow,
    InvalidSequence,
    Empty
}

#[derive(Clone)]
pub struct Utf8<const A: usize> {
    buf: array::Array<A, u8>,
    len: usize
}

impl<const A: usize> Utf8<A> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            buf: array::Array {
                buf: unsafe {
                    core::mem::MaybeUninit::uninit().assume_init()
                },
                len: 0
            },
            len: 0
        }
    }

    #[inline]
    pub const fn encode<B>(bytes: &[u8]) -> Self {
        let mut ret: Self = Self::new();
        let mut key: usize = 0;
        while key < bytes.len() && ret.buf.len < A {
            let byte = bytes[key];
            ret.buf.buf[key].write(byte);
            ret.buf.len += 1;
            if byte & 0b1100_0000 != 0b1000_0000 {
                ret.len += 1
            }
            key += 1;
        }
        ret
    }

    #[inline] 
    pub fn push_str(&mut self, s: &str) -> Result<()> {
        let old_buf_len: usize = self.buf.len();
        let old_len: usize = self.len;
        for c in s.chars() {
            if let Err(e) = self.push(c) {
                self.buf.len = old_buf_len;
                self.len = old_len;
                return Err(e)
            }
        }
        Ok(())
    }

    #[inline]
    pub const fn push(&mut self, c: char) -> Result<()> {
        let code: u32 = c as u32;
        let arr: &mut array::Array<A, u8> = &mut self.buf;
        let req: usize = if code <= 0x7f {
            1
        } else if code <= 0x7ff {
            2
        } else if code <= 0xffff {
            3
        } else {
            4
        };
        if arr.len() + req > A {
            return Err(Error::Overflow)
        }
        match req {
            1 => {
                let _ = arr.push(code as u8);
            },
            2 => {
                let _ = arr.push((0b1100_0000 | ((code >> 6) & 0x1F)) as u8);
                let _ = arr.push((0b1000_0000 | (code & 0x3F)) as u8);
            },
            3 => {
                let _ = arr.push((0b1110_0000 | ((code >> 12) & 0x0F)) as u8);
                let _ = arr.push((0b1000_0000 | ((code >> 6) & 0x3F)) as u8);
                let _ = arr.push((0b1000_0000 | (code & 0x3F)) as u8);
            },
            4 => {
                let _ = arr.push((0b1111_0000 | ((code >> 18) & 0x07)) as u8);
                let _ = arr.push((0b1000_0000 | ((code >> 12) & 0x3F)) as u8);
                let _ = arr.push((0b1000_0000 | ((code >> 6) & 0x3F)) as u8);
                let _ = arr.push((0b1000_0000 | (code & 0x3F)) as u8);
            },
            _ => unreachable!()
        }
        self.len += 1;
        Ok(())
    }

    #[inline]
    pub const fn pop(&mut self) -> Result<()> {
        let buf: &[u8] = self.buf.as_slice();
        let key: usize = self.buf.len();
        if key == 0 {
            return Err(Error::Empty)
        }
        let mut back: usize = 1;
        while back <= 4 {
            if key < back {
                break
            }
            let b: u8 = buf[key - back];
            if back == 1 && b & 0b1000_0000 == 0 {
                self.buf.len -= 1;
                self.len -= 1;
                return Ok(())
            } else if back == 2 && b & 0b1110_0000 == 0b1100_0000 {
                self.buf.len -= 2;
                self.len -= 1;
                return Ok(())
            } else if back == 3 && b & 0b1111_0000 == 0b1110_0000 {
                self.buf.len -= 3;
                self.len -= 1;
                return Ok(())
            } else if back == 4 && b & 0b1111_1000 == 0b1111_0000 {
                self.buf.len -= 4;
                self.len -= 1;
                return Ok(())
            }
            back += 1;
        }
        Err(Error::InvalidSequence)
    }

    pub fn cast<const B: usize>(self) -> Result<Utf8<B>> {
        let a: Utf8<A> = self;
        let a: &str = a.as_str();
        let mut b: Utf8<B> = Utf8::new();
        b.push_str(a)?;
        Ok(b)
    }

    pub fn as_str(&self) -> &str {
        let ret = self.buf.as_slice();
        ::core::str::from_utf8(ret)
            .ok()
            .ok_or(Error::InvalidSequence)
            .unwrap()
    }
}

impl<const T: usize> Default for Utf8<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const T: usize> TryFrom<&str> for Utf8<T> {
    type Error = Error;

    fn try_from(value: &str) -> ::core::result::Result<Self, Self::Error> {
        let mut s: Self = Self::new();
        s.push_str(value)?;
        Ok(s)
    }
}

impl<const T: usize> Eq for Utf8<T> {}
impl<const T: usize> PartialEq for Utf8<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.buf == other.buf
    }
}

impl<const T: usize> Ord for Utf8<T> {
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        let x: &str = self.as_str();
        let y: &str = other.as_str();
        x.cmp(y)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<const T: usize> PartialOrd for Utf8<T> {
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        let x: &str = self.as_str();
        let y: &str = other.as_str();
        let ret: ::core::cmp::Ordering = x.cmp(y);
        Some(ret)
    }
}

impl<const T: usize> ::core::fmt::Debug for Utf8<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f
            .debug_struct("Utf8")
            .field("len", &self.len)
            .field("buf_len", &self.buf.len)
            .field("as_str", &self.as_str())
            .finish()
    }
}

impl<const T: usize> ::core::fmt::Display for Utf8<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let s: &str = self.as_str();
        write!(f, "{}", s)
    }
}

#[test]
fn test() {
    let s: Utf8<32> = "Hello World".try_into().unwrap();
    let s_ok: Utf8<32> = "Hello World".try_into().unwrap();
    assert_eq!(s, s_ok);
}