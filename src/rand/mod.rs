use super::*;

pub struct Xoshiro<const T: usize = 4> {
    seed: [u32; T]
}

impl Xoshiro<4> {
    pub fn next<B>(&mut self, range: ::core::ops::Range<B>) -> B 
    where
        B: ops::Int {
        Self::r#gen(move || {
            let seed = self.seed.to_owned();
            let s = seed[0].wrapping_add(seed[3]);
            let s = s.rotate_left(7);
            let s = s.wrapping_add(self.seed[0]);
            let s = s as u64;
            let t = seed[1] << 9;
            seed[2] ^= self.seed[0];
            seed[3] ^= self.seed[1];
            seed[1] ^= self.seed[2];
            self.seed[0] ^= self.seed[3];
            self.seed[2] ^= t;
            self.seed[3] = self.seed[3].rotate_left(11);
            s.try_into().ok().unwrap()
        }, range)
    }
}

impl Xoshiro<8> {
    pub fn next<B>(&mut self, range: ::core::ops::Range<B>) -> B 
    where
        B: ops::Int {
        
    }
}

impl Xoshiro<16> {
    pub fn next<B>(&mut self, range: ::core::ops::Range<B>) -> B 
    where
        B: ops::Int {
        
    }
}

impl<const A: usize> Xoshiro<A> {
    fn roll<B>(x: B, k: B) -> B
    where
        B: ops::Int {
        (x << k) | (x >> (B::AS_32 - k))
    }

    fn r#gen<B, C>(rng: B, range: ::core::ops::Range<C>) -> C
    where
        B: Fn() -> C,
        C: ops::Int {
        let df = range.end.wrapping_sub(range.start);
        let mx = C::MAX;
        let up = (mx / df) * df;
        loop {
            let v = rng();
            if v < up {
                let v = (v % df) + range.start;
                return v
            }
        }
    }
}

impl From<[u8; 4]> for Xoshiro<4> {
    fn from(value: [u8; 16]) -> Self {
        let seed: [u8; 16] = value;
        let mut s: [u32; 4] = [0_u32; 4];
        for i in 0..4 {
            s[i] = u32::from_le_bytes([
                seed[i * 4],
                seed[i * 4 + 1],
                seed[i * 4 + 2],
                seed[i * 4 + 3]
            ]);
        }
        let seed: [u32; 4] = s;
        Self {
            seed
        }
    }
}