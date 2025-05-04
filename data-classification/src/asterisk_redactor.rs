use crate::Redactor;

/// Produces redactors that replace the original string with asterisks.
pub struct AsteriskRedactor {}

impl AsteriskRedactor {
    /// Creates a new instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Redactor for AsteriskRedactor {
    fn redact<'a>(&self, value: &str, output: Box<dyn FnOnce(&str) + 'a>) {
        const ASTERISKS: &str = "********************************";

        let len = value.len();
        if len < ASTERISKS.len() {
            output(&ASTERISKS[0..len]);
        } else {
            output("*".repeat(len).as_str());
        }
    }
}

impl Default for AsteriskRedactor {
    fn default() -> Self {
        Self::new()
    }
}
