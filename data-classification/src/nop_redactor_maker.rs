use crate::{Redactor, RedactorMaker};

/// Produces redactors that do not modify the original string.
pub struct NopRedactorMaker {
}

impl NopRedactorMaker {
    /// Creates a new instance.
    #[must_use]
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> RedactorMaker<'a> for NopRedactorMaker {
    fn make_redactor<F>(&self, mut output: F) -> Redactor<'a>
    where
        F: FnMut(&str) + 'a,
    {
        Redactor::new(move |s| {
            (output)(s);
        })
    }
}

impl Default for NopRedactorMaker {
    fn default() -> Self {
        Self::new()
    }
}
