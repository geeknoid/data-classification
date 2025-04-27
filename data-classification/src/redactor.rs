use std::fmt::Write;

/// The output sink used to emit data to redact.
pub struct Redactor<'a> {
    output: Box<dyn FnMut(&str) + 'a>,
}

impl<'a> Redactor<'a> {
    /// Creates a new redactor instance.
    ///
    /// Text written to the redactor is redirected to the provided output function, which
    /// is where redaction actually takes place.
    #[must_use]
    pub fn new<F>(output: F) -> Self
    where
        F: FnMut(&str) + 'a,
    {      
        Self {
            output: Box::new(output),
        }
    }

    /// Writes a string slice into this redactor.
    pub fn write_str(&mut self, str: &str) {
        (self.output)(str);
    }
}

impl Write for Redactor<'_> {
    fn write_str(&mut self, str: &str) -> std::fmt::Result {
        (self.output)(str);
        Ok(())
    }
}