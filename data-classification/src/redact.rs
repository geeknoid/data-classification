use crate::redactor::Redactor;

/// A trait for types that can produce a redacted version of themselves.
pub trait Redact {
    /// Converts the given value to a redacted form.
    fn externalize(&self, redactor: &mut Redactor);
}
