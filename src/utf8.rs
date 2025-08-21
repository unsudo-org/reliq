use super::*;

pub type Result<T> = ::core::result::Result<T, Error>;

pub enum Error {
    Overflow,
    IllegalByteSequence,
    Empty
}

#[derive(Clone)]
pub struct Utf8<const T: usize> {
    buf: array::Array<T, u8>,
    len: usize
}

impl<const T: usize> Utf8<T> {
    #[inline]
    pub const fn zero() -> Self {
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
    pub const fn encode(bytes: &[u8]) -> Self {
        let mut ret: Self = Self::zero();
        let mut key: usize = 0;
        while key < bytes.len() && ret.buf.len < T {
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
        let arr: &mut array::Array<T, u8> = &mut self.buf;
        let req: usize = if code <= 0x7f {
            1
        } else if code <= 0x7ff {
            2
        } else if code <= 0xffff {
            3
        } else {
            4
        };
        if arr.len() + req > T {
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
        Err(Error::IllegalByteSequence)
    }
}

