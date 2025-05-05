/// Represents types that can redact data.
pub trait Redactor {
    /// Redacts the given value and calls the output function with the redacted value.
    fn redact<'a>(&self, value: &str, output: &'a mut dyn FnMut(&str));

    /// The exact length of redacted strings, if they are constant.
    ///
    /// This can be used as a hint to optimize buffer allocations.
    fn exact_len(&self) -> Option<usize> {
        None
    }
}
