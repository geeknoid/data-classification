use crate::{Redactor, RedactorMaker};

/// Produces redactors that replace the original string with asterisks.
pub struct AsteriskRedactorMaker {
}

impl AsteriskRedactorMaker {
    /// Creates a new instance.
    #[must_use]
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> RedactorMaker<'a> for AsteriskRedactorMaker {
    fn make_redactor<F>(&self, mut output: F) -> Redactor<'a>
    where
        F: FnMut(&str) + 'a,
    {
        Redactor::new(move |s| {
            const ASTERISKS: &str = "********************************";

            let len = s.len();
            if len < ASTERISKS.len() {
                (output)(&ASTERISKS[0..len]);
            } else {
                (output)("*".repeat(len).as_str());
            }
        })
    }
}

impl Default for AsteriskRedactorMaker {
    fn default() -> Self {
        Self::new()
    }
}
