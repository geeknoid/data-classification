use crate::Redactor;

/// Produces redactors that simply erase the original string.
pub struct ErasingRedactor {}

impl ErasingRedactor {
    /// Creates a new instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Redactor for ErasingRedactor {
    fn redact<'a>(&self, _value: &str, _output: &'a mut dyn FnMut(&str)) {
        // nothing
    }

    fn exact_len(&self) -> Option<usize> {
        Some(0)
    }
}

impl Default for ErasingRedactor {
    fn default() -> Self {
        Self::new()
    }
}
