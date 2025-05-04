/// The output sink used to emit data to redact.
pub struct RedactionSink<'a> {
    output: Box<dyn FnOnce(&str) + 'a>,
}

impl<'a> RedactionSink<'a> {
    /// Creates a new redactor instance.
    ///
    /// Text written to the redactor is redirected to the provided output function, which
    /// is where redaction actually takes place.
    #[must_use]
    pub fn new(output: Box<dyn FnOnce(&str) + 'a>) -> Self {
        Self {
            output,
        }
    }

    /// Writes a string slice to be redacted.
    pub fn write_str(self, str: &str) {
        (self.output)(str);
    }
}
