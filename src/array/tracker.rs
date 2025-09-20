use super::*;

static mut NEXT_HANDLE: usize = 0;

pub type Trackable<const A: u8, B> = Array<A, Tracker<B>>;

/// # Brief
/// A wrapper to attach a `Handle` on. 
/// Enables the `Array` to track specific `T`s and remove them regardless of order.
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(::serde::Serialize)]
#[derive(::serde::Deserialize)]
pub struct Tracker<T>(usize, T);

impl<A, B, C> From<(B, C)> for Tracker<A>
where
    B: Into<usize>,
    C: Into<A> {
    fn from(value: (B, C)) -> Self {
        let x: usize = value.0.into();
        let y: A = value.1.into();
        Self(x, y)
    }
}

impl<T> ::core::ops::Deref for Tracker<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T> ::core::ops::DerefMut for Tracker<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

impl<const A: usize, B> Array<A, Tracker<B>> 
where
    B: Copy {
    pub fn get_handle<C>(&self, handle: C) -> Option<&Tracker<B>> 
    where
        C: Into<Handle> {
        let handle: Handle = handle.into();
        self.get(*handle)
    }

    pub fn get_mut_handle<C>(&mut self, handle: C) -> Option<&mut Tracker<B>> 
    where
        C: Into<Handle> {
        let handle: Handle = handle.into();
        self.get_mut(*handle)
    }

    #[inline]
    pub fn with_handle<C, D, E>(&self, handle: C, on_found: D) -> Option<E> 
    where
        C: Into<Handle>,
        D: FnOnce(&B) -> E {
        if let Some(item) = self.get_handle(handle) {
            return Some(on_found(item))
        }
        None
    }

    #[inline]
    pub fn with_mut_handle<C, D, E>(&mut self, handle: C, on_found: D) -> Option<E> 
    where
        C: Into<Handle>,
        D: FnOnce(&mut B) -> E {
        let handle: Handle = handle.into();
        if let Some(item) = self.get_mut_handle(handle) {
            return Some(on_found(item))
        }
        None
    }

    #[inline]
    pub fn push_tracked<C>(&mut self, item: C) -> Result<Handle>
    where
        C: Into<B> {
        let key = unsafe {
            NEXT_HANDLE += 1;
            NEXT_HANDLE
        };
        let item: Tracker<B> = (key, item).into();
        self.push(item)?;
        Ok(key.into())
    }

    #[inline]
    pub fn remove_handle<C>(&mut self, handle: C) -> Option<B>
    where
        C: Into<Handle> {
        let handle = handle.into();
        for k in 0..self.len() {
            if let Some(tracker) = self.get(k) {
                if tracker.0 == *handle {
                    let removed = self.remove_unsorted(k)?;
                    return Some(removed.1)
                }
            }
        }
        None
    }
}


fn hello_world(mut counts: Array<2, Tracker<u8>>) {
    let age = counts.push_tracked(48).unwrap();
    counts.remove_handle(age).unwrap();
}