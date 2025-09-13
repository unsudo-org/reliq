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

    pub fn with_item(mut self, item: B) -> Self {
        if let Some(Err(_)) = self.outcome {
            return self
        }
        if let Err(e) = self.content.push(item) {
            self.outcome = Some(Err(e));
        }
        self
    }

    pub fn build(self) -> Result<Array<A, B>> {
        if let Some(Err(e)) = self.outcome {
            return Err(e)
        }
        Ok(self.content)
    }
}