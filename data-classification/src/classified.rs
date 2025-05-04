use crate::redaction_sink::RedactionSink;

/// Represents a type that holds sensitive information.
pub trait Classified {
    /// Converts the given value to a redacted form.
    fn externalize(&self, redactor: RedactionSink);

    /// Returns the taxonomy of the data
    fn taxonomy(&self) -> &'static str;

    /// Returns the class of the data which is within the scope of the taxonomy.
    fn class(&self) -> &'static str;
}
