::modwire::expose!(
    pub hasher
    pub key
    pub val
);

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
    #[error("Overflow.")]
    Overflow
}

#[allow(deprecated)]
#[derive(Clone)]
pub struct Map<const A: usize, B, C, D = ::core::hash::SipHasher> 
where
    B: Key,
    C: Val,
    D: ::core::hash::Hasher {
    keys: [Option<B>; A],
    vals: [Option<C>; A],
    len: usize,
    hasher: ::core::marker::PhantomData<D>
}

impl<const A: usize, B, C, D> Map<A, B, C, D>
where
    B: Key,
    C: Val,
    D: Hasher {
    pub const fn new() -> Self {
        Self {
            keys: [None; A],
            vals: [None; A],
            len: 0,
            hasher: ::core::marker::PhantomData
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn cap(&self) -> usize {
        A
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, key: &B) -> Option<&C> {
        let mut position: usize = self.hash_index(key);
        for _ in 0..A {
            match &self.keys[position] {
                Some(k) if k == key => return self.vals[position].as_ref(),
                None => return None,
                _ => position = (position + 1) % A
            }
        }
        None
    }

    #[inline]
    pub fn insert(&mut self, key: B, data: C) -> Result<()> {
        if self.len >= A {
            return Err(Error::Overflow)
        }
        let mut position: usize = self.hash_index(&key);
        for _ in 0..A {
            match &self.keys[position] {
                Some(existing_key) if *existing_key == key => {
                    self.vals[position] = Some(data);
                    return Ok(())
                },
                None => {
                    self.keys[position] = Some(key);
                    self.vals[position] = Some(data);
                    self.len += 1;
                    return Ok(())
                },
                _ => position = (position + 1) % A
            }
        }
        Err(Error::Overflow)
    }

    #[inline]
    pub fn remove(&mut self, key: &B) -> Option<C> {
        let mut pos = self.hash_index(key);
        for _ in 0..A {
            match self.keys[pos] {
                Some(k) if k == *key => {
                    let old_val = self.vals[pos].take();
                    self.keys[pos] = None;
                    self.len -= 1;
                    let mut next = (pos + 1) % A;
                    while let Some(k) = self.keys[next] {
                        let val = self.vals[next].take().unwrap();
                        self.keys[next] = None;
                        self.len -= 1;
                        self.insert(k, val).unwrap();
                        next = (next + 1) % A;
                    }
                    return old_val;
                }
                None => return None,
                _ => pos = (pos + 1) % A,
            }
        }
        None
    }

    #[inline]
    fn hash_index(&self, key: &B) -> usize {
        let mut hasher: D = D::default();
        key.hash(&mut hasher);
        (
            hasher.finish() as usize
        ) % A
    }
}

impl<const A: usize, B, C, D> Default for Map<A, B, C, D> 
where
    B: Key,
    C: Val,
    D: Hasher {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl<const A: usize, B, C, D> ::serde::Serialize for Map<A, B, C, D>
where
    B: Key,
    B: ::serde::Serialize,
    C: Val,
    C: ::serde::Serialize,
    D: Hasher {
    fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut ret: Vec<(&B, &C)> = Vec::with_capacity(self.len);
        for i in 0..A {
            if let (Some(k), Some(v)) = (&self.keys[i], &self.vals[i]) {
                ret.push((k, v));
            }
        }
        ret.serialize(serializer)
    }
}

#[cfg(feature = "std")]
impl<'de, const A: usize, B, C, D> ::serde::Deserialize<'de> for Map<A, B, C, D>
where
    B: Key,
    B: ::serde::Deserialize<'de>,
    C: Val,
    C: ::serde::Deserialize<'de>,
    D: Hasher {
    fn deserialize<De>(deserializer: De) -> ::core::result::Result<Self, De::Error>
    where
        De: serde::Deserializer<'de> {
        let entries: Vec<(B, C)> = ::serde::Deserialize::deserialize(deserializer)?;
        if entries.len() > A {
            return Err(serde::de::Error::custom("Array overflow during deserialization"));
        }
        let mut map = Map::<A, B, C, D>::new();
        for (k, v) in entries {
            map
                .insert(k, v)
                .ok()
                .ok_or(::serde::de::Error::custom("out of space"))?;
        }
        Ok(map)
    }
}