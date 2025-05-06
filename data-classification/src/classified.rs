use crate::redaction_sink::RedactionSink;

/// Represents a type that holds sensitive information.
pub trait Classified {
    /// Converts the given value to a redacted form.
    fn externalize(&self, redactor: RedactionSink);

    /// Returns the taxonomy of the data class.
    fn taxonomy(&self) -> &'static str;

    /// Returns the name of the data class.
    fn class(&self) -> &'static str;
}
