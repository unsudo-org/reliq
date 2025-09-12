#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Handle(usize);

impl<T> From<T> for Handle 
where
    T: Into<usize> {
    fn from(value: T) -> Self {
        let value: usize = value.into();
        Self(value)
    }
}

impl ::core::ops::Deref for Handle {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}