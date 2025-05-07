use crate::Redactor;
use data_classification::ClassId;

/// Produces redactors that do not modify the original string.
#[derive(Clone)]
pub struct NopRedactor {}

impl NopRedactor {
    /// Creates a new instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Redactor for NopRedactor {
    fn redact<'a>(&self, _: &ClassId, value: &str, output: &'a mut dyn FnMut(&str)) {
        output(value);
    }
}

impl Default for NopRedactor {
    fn default() -> Self {
        Self::new()
    }
}
