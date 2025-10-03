use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct Builder<const A: usize, B>
where
    B: Clone,
    B: Copy {
    content: Array<A, B>,
    outcome: Option<Result<()>>
}

impl<const A: usize, B> Builder<A, B> 
where
    B: Clone,
    B: Copy {
    pub(super) fn new() -> Self {
        Self {
            content: Array::default(),
            outcome: None
        }
    }

    #[inline]
    pub fn push<C>(mut self, item: C) -> Self 
    where
        C: Into<B> {
        let item: B = item.into();
        if let Some(Err(_)) = self.outcome {
            return self
        }
        if self.content.len >= A {
            self.outcome = Some(Err(Error::Overflow))
        }
        self.content.buf[self.content.len].write(item);
        self.content.len += 1;
        self
    }

    #[inline]
    pub fn finish(self) -> Result<Array<A, B>> {
        if let Some(Err(e)) = self.outcome {
            return Err(e)
        }
        Ok(self.content)
    }
}