use super::*;

#[cfg(feature = "ink")]
impl<const A: usize, B> ::scale::Encode for Array<A, B>
where
    B: Default,
    B: Copy,
    B: ::scale::Encode {
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        let len: u32 = self.len as u32;
        ::scale::Compact(len).encode_to(dest);
        for item in self.as_slice() {
            item.encode_to(dest);
        }
    }
}

#[cfg(feature = "ink")]
impl<const A: usize, B> ::scale::Decode for Array<A, B>
where
    B: Default,
    B: Copy,
    B: ::scale::Decode {
    fn decode<I: scale::Input>(input: &mut I) -> ::core::result::Result<Self, ::scale::Error> {
        let len: u32 = <::scale::Compact<u32>>::decode(input)?.0;
        if (len as usize) > A {
            let e: ::scale::Error = "".into(); 
            return Err(e)
        }
        let mut arr = Self::default();
        for _ in 0..len {
            let item = B::decode(input)?;
            arr.push(item).expect("");
        }
        Ok(arr)
    }
}