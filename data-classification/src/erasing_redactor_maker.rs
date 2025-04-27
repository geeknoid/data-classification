use crate::{Redactor, RedactorMaker};

/// Produces redactors that simply erase the original string.
pub struct ErasingRedactorMaker {
}

impl ErasingRedactorMaker {
    /// Creates a new instance.
    #[must_use]
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> RedactorMaker<'a> for ErasingRedactorMaker {
    fn make_redactor<F>(&self, _: F) -> Redactor<'a>
    where
        F: FnMut(&str) + 'a,
    {
        Redactor::new(|_| { })
    }

    fn exact_len(&self) -> Option<usize> {
        Some(0)
    }
}

impl Default for ErasingRedactorMaker {
    fn default() -> Self {
        Self::new()
    }
}
