/// The output sink used to emit data to redact.
pub struct RedactionSink<'a> {
    output: &'a mut dyn FnMut(&str),
}

impl<'a> RedactionSink<'a> {
    /// Creates a new redactor instance.
    ///
    /// Text written to the redactor is redirected to the provided output function, which
    /// is where redaction actually takes place.
    #[must_use]
    pub fn new(output: &'a mut dyn FnMut(&str)) -> Self {
        Self { output }
    }

    /// Writes a string slice to be redacted.
    pub fn write_str(self, str: &str) {
        (self.output)(str);
    }
}
