use crate::Redactor;

/// A trait for types that can produce a redactor.
pub trait RedactorMaker<'a> {
    /// Creates a new redactor.
    #[must_use]
    fn make_redactor<F>(&self, output: F) -> Redactor<'a>
    where
        F: FnMut(&str) + 'a;

    /// The exact length of redacted strings, if they are constant.
    ///
    /// This can be used as a hint to optimize buffer allocations.
    fn exact_len(&self) -> Option<usize> {
        None
    }
}